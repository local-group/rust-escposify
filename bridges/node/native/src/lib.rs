#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult, Module};
use neon::js::JsString;

fn hello(call: Call) -> JsResult<JsString> {
    println!("env={}", env!("NEON_NODE_ABI"));
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}

register_module!(m, {
    m.export("hello", hello)
});
