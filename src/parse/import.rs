use std::{ffi::OsStr, fs, path::Path};

use peekmore::PeekMore;

use crate::{error::SassResult, Token};

use crate::lexer::Lexer;

use super::{Parser, Stmt};

impl<'a> Parser<'a> {
    pub(super) fn import(&mut self) -> SassResult<Vec<Stmt>> {
        self.whitespace();
        let mut file_name = String::new();
        let next = match self.toks.next() {
            Some(v) => v,
            None => todo!("expected input after @import"),
        };
        match next.kind {
            q @ '"' | q @ '\'' => {
                file_name.push_str(
                    &self
                        .parse_quoted_string(q)?
                        .node
                        .unquote()
                        .to_css_string(self.span_before)?,
                );
            }
            _ => return Err(("Expected string.", next.pos()).into()),
        }
        if let Some(t) = self.toks.peek() {
            if t.kind == ';' {
                self.toks.next();
            }
        }

        self.whitespace();

        let path: &Path = file_name.as_ref();

        let path_buf = if path.is_absolute() {
            // todo: test for absolute path imports
            path.into()
        } else {
            self.path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(path)
        };
        // todo: will panic if path ended in `..`
        let name = path_buf.file_name().unwrap();
        if path_buf.extension() == Some(OsStr::new(".css")) {
            // || name.starts_with("http://") || name.starts_with("https://") {
            todo!("css imports")
        }

        let paths = [
            path_buf.with_file_name(name).with_extension("scss"),
            path_buf.with_file_name(format!("_{}.scss", name.to_str().unwrap())),
            path_buf.clone(),
            path_buf.clone().join("index.scss"),
            path_buf.clone().join("_index.scss"),
        ];

        for name in &paths {
            if name.is_file() {
                let file = self.map.add_file(
                    name.to_string_lossy().into(),
                    String::from_utf8(fs::read(name)?)?,
                );

                return Parser {
                    toks: &mut Lexer::new(&file)
                        .collect::<Vec<Token>>()
                        .into_iter()
                        .peekmore(),
                    map: &mut self.map,
                    path: name.as_ref(),
                    scopes: &mut self.scopes,
                    global_scope: &mut self.global_scope,
                    super_selectors: self.super_selectors,
                    span_before: file.span.subspan(0, 0),
                    content: self.content.clone(),
                    in_mixin: self.in_mixin,
                    in_function: self.in_function,
                    in_control_flow: self.in_control_flow,
                    at_root: self.at_root,
                    at_root_has_selector: self.at_root_has_selector,
                }
                .parse();
            }
        }

        Ok(Vec::new())
    }
}
