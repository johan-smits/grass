use codemap::Span;

use crate::{common::unvendor, error::SassResult, parse::Parser, utils::is_name, Token};

use super::{
    Attribute, Combinator, ComplexSelector, ComplexSelectorComponent, CompoundSelector, Namespace,
    Pseudo, QualifiedName, SelectorList, SimpleSelector,
};

#[derive(PartialEq)]
enum DevouredWhitespace {
    /// Some whitespace was found
    Whitespace,
    /// A newline and potentially other whitespace was found
    Newline,
    /// No whitespace was found
    None,
}

impl DevouredWhitespace {
    fn found_whitespace(&mut self) {
        if self == &Self::None {
            *self = Self::Whitespace;
        }
    }

    fn found_newline(&mut self) {
        *self = Self::Newline;
    }
}

/// Pseudo-class selectors that take unadorned selectors as arguments.
const SELECTOR_PSEUDO_CLASSES: [&str; 8] = [
    "not",
    "matches",
    "is",
    "current",
    "any",
    "has",
    "host",
    "host-context",
];

/// Pseudo-element selectors that take unadorned selectors as arguments.
const SELECTOR_PSEUDO_ELEMENTS: [&str; 1] = ["slotted"];

pub(crate) struct SelectorParser<'a, 'b, 'c> {
    /// Whether this parser allows the parent selector `&`.
    allows_parent: bool,

    /// Whether this parser allows placeholder selectors beginning with `%`.
    allows_placeholder: bool,

    parser: &'a mut Parser<'b, 'c>,

    span: Span,
}

impl<'a, 'b, 'c> SelectorParser<'a, 'b, 'c> {
    pub fn new(
        parser: &'a mut Parser<'b, 'c>,
        allows_parent: bool,
        allows_placeholder: bool,
        span: Span,
    ) -> Self {
        Self {
            allows_parent,
            allows_placeholder,
            parser,
            span,
        }
    }

    pub fn parse(mut self) -> SassResult<SelectorList> {
        let tmp = self.parse_selector_list()?;
        if self.parser.toks.peek().is_some() {
            return Err(("expected selector.", self.span).into());
        }
        Ok(tmp)
    }

    fn parse_selector_list(&mut self) -> SassResult<SelectorList> {
        let mut components = vec![self.parse_complex_selector(false)?];

        self.parser.whitespace();

        let mut line_break = false;

        while let Some(Token { kind: ',', .. }) = self.parser.toks.peek() {
            self.parser.toks.next();
            line_break = self.eat_whitespace() == DevouredWhitespace::Newline || line_break;
            match self.parser.toks.peek() {
                Some(Token { kind: ',', .. }) => continue,
                Some(..) => {}
                None => break,
            }
            components.push(self.parse_complex_selector(line_break)?);

            line_break = false;
        }

        Ok(SelectorList {
            components,
            span: self.span,
        })
    }

    fn eat_whitespace(&mut self) -> DevouredWhitespace {
        let mut whitespace_devoured = DevouredWhitespace::None;
        while let Some(tok) = self.parser.toks.peek() {
            match tok.kind {
                ' ' | '\t' => whitespace_devoured.found_whitespace(),
                '\n' => whitespace_devoured.found_newline(),
                _ => break,
            }
            self.parser.toks.next();
        }

        whitespace_devoured
    }

    /// Consumes a complex selector.
    ///
    /// If `line_break` is `true`, that indicates that there was a line break
    /// before this selector.
    fn parse_complex_selector(&mut self, line_break: bool) -> SassResult<ComplexSelector> {
        let mut components = Vec::new();

        loop {
            self.parser.whitespace();

            // todo: can we do while let Some(..) = self.parser.toks.peek() ?
            match self.parser.toks.peek() {
                Some(Token { kind: '+', .. }) => {
                    self.parser.toks.next();
                    components.push(ComplexSelectorComponent::Combinator(
                        Combinator::NextSibling,
                    ));
                }
                Some(Token { kind: '>', .. }) => {
                    self.parser.toks.next();
                    components.push(ComplexSelectorComponent::Combinator(Combinator::Child));
                }
                Some(Token { kind: '~', .. }) => {
                    self.parser.toks.next();
                    components.push(ComplexSelectorComponent::Combinator(
                        Combinator::FollowingSibling,
                    ));
                }
                Some(Token { kind: '[', .. })
                | Some(Token { kind: '.', .. })
                | Some(Token { kind: '#', .. })
                | Some(Token { kind: '%', .. })
                | Some(Token { kind: ':', .. })
                // todo: ampersand?
                | Some(Token { kind: '&', .. })
                | Some(Token { kind: '*', .. })
                | Some(Token { kind: '|', .. }) => {
                    components.push(ComplexSelectorComponent::Compound(
                        self.parse_compound_selector()?,
                    ));
                    if let Some(Token { kind: '&', .. }) = self.parser.toks.peek() {
                        return Err(("\"&\" may only used at the beginning of a compound selector.", self.span).into());
                    }
                }
                Some(..) => {
                    if !self.parser.looking_at_identifier() {
                        break;
                    }
                    components.push(ComplexSelectorComponent::Compound(
                        self.parse_compound_selector()?,
                    ));
                    if let Some(Token { kind: '&', .. }) = self.parser.toks.peek() {
                        return Err(("\"&\" may only used at the beginning of a compound selector.", self.span).into());
                    }
                }
                None => break,
            }
        }

        if components.is_empty() {
            return Err(("expected selector.", self.span).into());
        }

        Ok(ComplexSelector::new(components, line_break))
    }

    fn parse_compound_selector(&mut self) -> SassResult<CompoundSelector> {
        let mut components = vec![self.parse_simple_selector(None)?];

        while let Some(Token { kind, .. }) = self.parser.toks.peek() {
            if !is_simple_selector_start(kind) {
                break;
            }

            components.push(self.parse_simple_selector(Some(false))?);
        }

        Ok(CompoundSelector { components })
    }

    fn looking_at_identifier_body(&mut self) -> bool {
        matches!(self.parser.toks.peek(), Some(t) if is_name(t.kind) || t.kind == '\\')
    }

    /// Consumes a simple selector.
    ///
    /// If `allows_parent` is `Some`, this will override `self.allows_parent`. If `allows_parent`
    /// is `None`, it will fallback to `self.allows_parent`.
    fn parse_simple_selector(&mut self, allows_parent: Option<bool>) -> SassResult<SimpleSelector> {
        match self.parser.toks.peek() {
            Some(Token { kind: '[', .. }) => self.parse_attribute_selector(),
            Some(Token { kind: '.', .. }) => self.parse_class_selector(),
            Some(Token { kind: '#', .. }) => self.parse_id_selector(),
            Some(Token { kind: '%', .. }) => {
                if !self.allows_placeholder {
                    return Err(("Placeholder selectors aren't allowed here.", self.span).into());
                }
                self.parse_placeholder_selector()
            }
            Some(Token { kind: ':', .. }) => self.parse_pseudo_selector(),
            Some(Token { kind: '&', .. }) => {
                let allows_parent = allows_parent.unwrap_or(self.allows_parent);
                if !allows_parent {
                    return Err(("Parent selectors aren't allowed here.", self.span).into());
                }

                self.parse_parent_selector()
            }
            _ => self.parse_type_or_universal_selector(),
        }
    }

    fn parse_attribute_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.next();
        Ok(SimpleSelector::Attribute(Box::new(Attribute::from_tokens(
            self.parser,
        )?)))
    }

    fn parse_class_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.next();
        Ok(SimpleSelector::Class(self.parser.parse_identifier()?.node))
    }

    fn parse_id_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.next();
        Ok(SimpleSelector::Id(self.parser.parse_identifier()?.node))
    }

    fn parse_pseudo_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.next();
        let element = match self.parser.toks.peek() {
            Some(Token { kind: ':', .. }) => {
                self.parser.toks.next();
                true
            }
            _ => false,
        };

        let name = self.parser.parse_identifier()?;

        match self.parser.toks.peek() {
            Some(Token { kind: '(', .. }) => self.parser.toks.next(),
            _ => {
                return Ok(SimpleSelector::Pseudo(Pseudo {
                    is_class: !element && !is_fake_pseudo_element(&name),
                    name: name.node,
                    selector: None,
                    is_syntactic_class: !element,
                    argument: None,
                    span: self.span,
                }));
            }
        };

        self.parser.whitespace();

        let unvendored = unvendor(&name);

        let mut argument: Option<Box<str>> = None;
        let mut selector: Option<Box<SelectorList>> = None;

        if element {
            // todo: lowercase?
            if SELECTOR_PSEUDO_ELEMENTS.contains(&unvendored) {
                selector = Some(Box::new(self.parse_selector_list()?));
                self.parser.whitespace();
            } else {
                argument = Some(
                    self.parser
                        .declaration_value(true, false, true)?
                        .into_boxed_str(),
                );
            }

            self.parser.expect_char(')')?;
        } else if SELECTOR_PSEUDO_CLASSES.contains(&unvendored) {
            selector = Some(Box::new(self.parse_selector_list()?));
            self.parser.whitespace();
            self.parser.expect_char(')')?;
        } else if unvendored == "nth-child" || unvendored == "nth-last-child" {
            let mut this_arg = self.parse_a_n_plus_b()?;
            let found_whitespace = self.parser.whitespace();
            #[allow(clippy::match_same_arms)]
            match (found_whitespace, self.parser.toks.peek()) {
                (_, Some(Token { kind: ')', .. })) => {}
                (true, _) => {
                    self.expect_identifier("of")?;
                    this_arg.push_str(" of");
                    self.parser.whitespace();
                    selector = Some(Box::new(self.parse_selector_list()?));
                }
                _ => {}
            }
            self.parser.expect_char(')')?;
            argument = Some(this_arg.into_boxed_str());
        } else {
            argument = Some(
                self.parser
                    .declaration_value(true, false, true)?
                    .trim_end()
                    .to_owned()
                    .into_boxed_str(),
            );

            self.parser.expect_char(')')?;
        }

        Ok(SimpleSelector::Pseudo(Pseudo {
            is_class: !element && !is_fake_pseudo_element(&name),
            name: name.node,
            selector,
            is_syntactic_class: !element,
            argument,
            span: self.span,
        }))
    }

    fn parse_parent_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.next();
        let suffix = if self.looking_at_identifier_body() {
            Some(self.parser.parse_identifier()?.node)
        } else {
            None
        };
        Ok(SimpleSelector::Parent(suffix))
    }

    fn parse_placeholder_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.next();
        Ok(SimpleSelector::Placeholder(
            self.parser.parse_identifier()?.node,
        ))
    }

    /// Consumes a type selector or a universal selector.
    ///
    /// These are combined because either one could start with `*`.
    fn parse_type_or_universal_selector(&mut self) -> SassResult<SimpleSelector> {
        self.parser.toks.peek();

        match self.parser.toks.peek() {
            Some(Token { kind: '*', pos }) => {
                self.parser.span_before = self.parser.span_before.merge(pos);
                self.parser.toks.next();
                if let Some(Token { kind: '|', .. }) = self.parser.toks.peek() {
                    self.parser.toks.next();
                    if let Some(Token { kind: '*', .. }) = self.parser.toks.peek() {
                        self.parser.toks.next();
                        return Ok(SimpleSelector::Universal(Namespace::Asterisk));
                    }

                    return Ok(SimpleSelector::Type(QualifiedName {
                        ident: self.parser.parse_identifier()?.node,
                        namespace: Namespace::Asterisk,
                    }));
                }

                return Ok(SimpleSelector::Universal(Namespace::None));
            }
            Some(Token { kind: '|', pos }) => {
                self.parser.span_before = self.parser.span_before.merge(pos);
                self.parser.toks.next();
                match self.parser.toks.peek() {
                    Some(Token { kind: '*', .. }) => {
                        self.parser.toks.next();
                        return Ok(SimpleSelector::Universal(Namespace::Empty));
                    }
                    _ => {
                        return Ok(SimpleSelector::Type(QualifiedName {
                            ident: self.parser.parse_identifier()?.node,
                            namespace: Namespace::Empty,
                        }));
                    }
                }
            }
            _ => {}
        }

        let name_or_namespace = self.parser.parse_identifier()?.node;

        Ok(match self.parser.toks.peek() {
            Some(Token { kind: '|', .. }) => {
                self.parser.toks.next();
                if let Some(Token { kind: '*', .. }) = self.parser.toks.peek() {
                    self.parser.toks.next();
                    SimpleSelector::Universal(Namespace::Other(name_or_namespace.into_boxed_str()))
                } else {
                    SimpleSelector::Type(QualifiedName {
                        ident: self.parser.parse_identifier()?.node,
                        namespace: Namespace::Other(name_or_namespace.into_boxed_str()),
                    })
                }
            }
            Some(..) | None => SimpleSelector::Type(QualifiedName {
                ident: name_or_namespace,
                namespace: Namespace::None,
            }),
        })
    }

    /// Consumes an [`An+B` production][An+B] and returns its text.
    ///
    /// [An+B]: https://drafts.csswg.org/css-syntax-3/#anb-microsyntax
    fn parse_a_n_plus_b(&mut self) -> SassResult<String> {
        let mut buf = String::new();

        match self.parser.toks.peek() {
            Some(Token { kind: 'e', .. }) | Some(Token { kind: 'E', .. }) => {
                self.expect_identifier("even")?;
                return Ok("even".to_owned());
            }
            Some(Token { kind: 'o', .. }) | Some(Token { kind: 'O', .. }) => {
                self.expect_identifier("odd")?;
                return Ok("odd".to_owned());
            }
            Some(t @ Token { kind: '+', .. }) | Some(t @ Token { kind: '-', .. }) => {
                buf.push(t.kind);
                self.parser.toks.next();
            }
            _ => {}
        }

        match self.parser.toks.peek() {
            Some(t) if t.kind.is_ascii_digit() => {
                while let Some(t) = self.parser.toks.peek() {
                    if !t.kind.is_ascii_digit() {
                        break;
                    }
                    buf.push(t.kind);
                    self.parser.toks.next();
                }
                self.parser.whitespace();
                if let Some(t) = self.parser.toks.peek() {
                    if t.kind != 'n' && t.kind != 'N' {
                        return Ok(buf);
                    }
                    self.parser.toks.next();
                }
            }
            Some(t) => {
                if t.kind == 'n' || t.kind == 'N' {
                    self.parser.toks.next();
                } else {
                    return Err(("Expected \"n\".", self.span).into());
                }
            }
            None => return Err(("expected more input.", self.span).into()),
        }

        buf.push('n');

        self.parser.whitespace();

        if let Some(t @ Token { kind: '+', .. }) | Some(t @ Token { kind: '-', .. }) =
            self.parser.toks.peek()
        {
            buf.push(t.kind);
            self.parser.toks.next();
            self.parser.whitespace();
            match self.parser.toks.peek() {
                Some(t) if !t.kind.is_ascii_digit() => {
                    return Err(("Expected a number.", self.span).into())
                }
                None => return Err(("Expected a number.", self.span).into()),
                Some(..) => {}
            }

            while let Some(t) = self.parser.toks.peek() {
                if !t.kind.is_ascii_digit() {
                    break;
                }
                buf.push(t.kind);
                self.parser.toks.next();
            }
        }
        Ok(buf)
    }

    fn expect_identifier(&mut self, s: &str) -> SassResult<()> {
        let mut ident = self.parser.parse_identifier_no_interpolation(false)?.node;
        ident.make_ascii_lowercase();
        if ident == s {
            Ok(())
        } else {
            Err((format!("Expected \"{}\".", s), self.span).into())
        }
    }
}

/// Returns whether `c` can start a simple selector other than a type
/// selector.
fn is_simple_selector_start(c: char) -> bool {
    matches!(c, '*' | '[' | '.' | '#' | '%' | ':')
}

/// Returns whether `name` is the name of a pseudo-element that can be written
/// with pseudo-class syntax (`:before`, `:after`, `:first-line`, or
/// `:first-letter`)
fn is_fake_pseudo_element(name: &str) -> bool {
    match name.as_bytes().first() {
        Some(b'a') | Some(b'A') => name.to_ascii_lowercase() == "after",
        Some(b'b') | Some(b'B') => name.to_ascii_lowercase() == "before",
        Some(b'f') | Some(b'F') => matches!(
            name.to_ascii_lowercase().as_str(),
            "first-line" | "first-letter"
        ),
        _ => false,
    }
}
