#[macro_use]
extern crate neon;
extern crate escposify;

use neon::vm::{Call, JsResult, Module};
use neon::js::{JsString, JsUndefined};

use escposify::printer::Printer;
use escposify::device::File;
use escposify::device::Network;


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
