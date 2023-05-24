#[macro_use]
extern crate neon;
extern crate snbc;

use neon::vm::{Call, JsResult, Module};
use neon::js::{JsString, JsUndefined};

use snbc::printer::Printer;
use snbc::device::File;
use snbc::device::Network;


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
