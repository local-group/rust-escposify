#[macro_use]
extern crate neon;
extern crate posify;

use neon::vm::{Call, JsResult, Module};
use neon::js::{JsString, JsUndefined};

use posify::printer::Printer;
use posify::device::File;
use posify::device::Network;


fn print(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let arguments = call.arguments;
    Ok(JsUndefined::new())
    // println!("arguments = {:?}", arguments.get(scope, 0).unwrap());
}


register_module!(m, {
    try!(m.export("print", print));
    Ok(())
});
