use std::fmt;

use crate::{
    atrule::keyframes::{Keyframes, KeyframesSelector},
    error::SassResult,
    lexer::Lexer,
    parse::Stmt,
    Token,
};

use super::{common::ContextFlags, Parser};

impl fmt::Display for KeyframesSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyframesSelector::To => f.write_str("to"),
            KeyframesSelector::From => f.write_str("from"),
            KeyframesSelector::Percent(p) => write!(f, "{}%", p),
        }
    }
}

struct KeyframesSelectorParser<'a, 'b, 'c> {
    parser: &'a mut Parser<'b, 'c>,
}

impl<'a, 'b, 'c> KeyframesSelectorParser<'a, 'b, 'c> {
    pub fn new(parser: &'a mut Parser<'b, 'c>) -> Self {
        Self { parser }
    }

    fn parse_keyframes_selector(&mut self) -> SassResult<Vec<KeyframesSelector>> {
        let mut selectors = Vec::new();
        self.parser.whitespace_or_comment();
        while let Some(tok) = self.parser.toks.peek() {
            match tok.kind {
                't' | 'T' => {
                    let mut ident = self.parser.parse_identifier()?;
                    ident.node.make_ascii_lowercase();
                    if ident.node == "to" {
                        selectors.push(KeyframesSelector::To);
                    } else {
                        return Err(("Expected \"to\" or \"from\".", tok.pos).into());
                    }
                }
                'f' | 'F' => {
                    let mut ident = self.parser.parse_identifier()?;
                    ident.node.make_ascii_lowercase();
                    if ident.node == "from" {
                        selectors.push(KeyframesSelector::From);
                    } else {
                        return Err(("Expected \"to\" or \"from\".", tok.pos).into());
                    }
                }
                '0'..='9' => {
                    let mut num = self.parser.parse_whole_number();

                    if let Some(Token { kind: '.', .. }) = self.parser.toks.peek() {
                        self.parser.toks.next();
                        num.push('.');
                        num.push_str(&self.parser.parse_whole_number());
                    }

                    self.parser.expect_char('%')?;

                    selectors.push(KeyframesSelector::Percent(num.into_boxed_str()));
                }
                '{' => break,
                '\\' => todo!("escaped chars in @keyframes selector"),
                _ => return Err(("Expected \"to\" or \"from\".", tok.pos).into()),
            }
            self.parser.whitespace_or_comment();
            if let Some(Token { kind: ',', .. }) = self.parser.toks.peek() {
                self.parser.toks.next();
                self.parser.whitespace_or_comment();
            } else {
                break;
            }
        }
        Ok(selectors)
    }
}

impl<'a, 'b> Parser<'a, 'b> {
    fn parse_keyframes_name(&mut self) -> SassResult<String> {
        let mut name = String::new();
        self.whitespace_or_comment();
        while let Some(tok) = self.toks.next() {
            match tok.kind {
                '#' => {
                    if self.consume_char_if_exists('{') {
                        name.push_str(&self.parse_interpolation_as_string()?);
                    } else {
                        name.push('#');
                    }
                }
                ' ' | '\n' | '\t' => {
                    self.whitespace();
                    name.push(' ');
                }
                '{' => {
                    // todo: we can avoid the reallocation by trimming before emitting
                    // (in `output.rs`)
                    return Ok(name.trim().to_owned());
                }
                _ => name.push(tok.kind),
            }
        }
        Err(("expected \"{\".", self.span_before).into())
    }

    pub(super) fn parse_keyframes_selector(
        &mut self,
        mut string: String,
    ) -> SassResult<Vec<KeyframesSelector>> {
        let mut span = if let Some(tok) = self.toks.peek() {
            tok.pos()
        } else {
            return Err(("expected \"{\".", self.span_before).into());
        };

        self.span_before = span;

        while let Some(tok) = self.toks.next() {
            span = span.merge(tok.pos());
            match tok.kind {
                '#' => {
                    if self.consume_char_if_exists('{') {
                        string.push_str(
                            &self
                                .parse_interpolation()?
                                .to_css_string(span, self.options.is_compressed())?,
                        );
                    } else {
                        string.push('#');
                    }
                }
                ',' => {
                    while let Some(c) = string.pop() {
                        if c == ' ' || c == ',' {
                            continue;
                        }
                        string.push(c);
                        string.push(',');
                        break;
                    }
                }
                '/' => {
                    if self.toks.peek().is_none() {
                        return Err(("Expected selector.", tok.pos()).into());
                    }
                    self.parse_comment()?;
                    self.whitespace();
                    string.push(' ');
                }
                '{' => {
                    let sel_toks: Vec<Token> =
                        string.chars().map(|x| Token::new(span, x)).collect();

                    let selector = KeyframesSelectorParser::new(&mut Parser {
                        toks: &mut Lexer::new(sel_toks),
                        map: self.map,
                        path: self.path,
                        scopes: self.scopes,
                        global_scope: self.global_scope,
                        super_selectors: self.super_selectors,
                        span_before: self.span_before,
                        content: self.content,
                        flags: self.flags,
                        at_root: self.at_root,
                        at_root_has_selector: self.at_root_has_selector,
                        extender: self.extender,
                        content_scopes: self.content_scopes,
                        options: self.options,
                        modules: self.modules,
                        module_config: self.module_config,
                    })
                    .parse_keyframes_selector()?;

                    return Ok(selector);
                }
                c => string.push(c),
            }
        }

        Err(("expected \"{\".", span).into())
    }

    pub(super) fn parse_keyframes(&mut self, rule: String) -> SassResult<Stmt> {
        if self.flags.in_function() {
            return Err(("This at-rule is not allowed here.", self.span_before).into());
        }

        let name = self.parse_keyframes_name()?;

        self.whitespace();

        let body = Parser {
            toks: self.toks,
            map: self.map,
            path: self.path,
            scopes: self.scopes,
            global_scope: self.global_scope,
            super_selectors: self.super_selectors,
            span_before: self.span_before,
            content: self.content,
            flags: self.flags | ContextFlags::IN_KEYFRAMES,
            at_root: false,
            at_root_has_selector: self.at_root_has_selector,
            extender: self.extender,
            content_scopes: self.content_scopes,
            options: self.options,
            modules: self.modules,
            module_config: self.module_config,
        }
        .parse_stmt()?;

        Ok(Stmt::Keyframes(Box::new(Keyframes { rule, name, body })))
    }
}
