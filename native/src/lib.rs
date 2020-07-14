#[macro_use]
extern crate neon;

use std::str;
use neon::prelude::*;
use neon::register_module;

fn aaa(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello world!"))
}

register_module!(mut cx, {
    cx.export_function("aaa", aaa)
});
