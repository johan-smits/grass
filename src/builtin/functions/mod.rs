// A reference to the parser is only necessary for some functions
#![allow(unused_variables)]

use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use once_cell::sync::Lazy;

use crate::{args::CallArgs, error::SassResult, parse::Parser, value::Value};

#[macro_use]
mod macros;

pub mod color;
pub mod list;
pub mod map;
pub mod math;
pub mod meta;
pub mod selector;
pub mod string;

pub(crate) type GlobalFunctionMap = HashMap<&'static str, Builtin>;

static FUNCTION_COUNT: AtomicUsize = AtomicUsize::new(0);

// TODO: impl Fn
#[derive(Clone)]
pub(crate) struct Builtin(pub fn(CallArgs, &mut Parser) -> SassResult<Value>, usize);

impl Builtin {
    pub fn new(body: fn(CallArgs, &mut Parser) -> SassResult<Value>) -> Builtin {
        let count = FUNCTION_COUNT.fetch_add(1, Ordering::Relaxed);
        Self(body, count)
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Builtin {}

pub(crate) static GLOBAL_FUNCTIONS: Lazy<GlobalFunctionMap> = Lazy::new(|| {
    let mut m = HashMap::new();
    color::declare(&mut m);
    list::declare(&mut m);
    map::declare(&mut m);
    math::declare(&mut m);
    meta::declare(&mut m);
    selector::declare(&mut m);
    string::declare(&mut m);
    m
});
