#[macro_use]
extern crate neon;

#[macro_use]
extern crate serde_json;

extern crate glob;

use glob::glob;
use neon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

fn inner_main(path: &str) -> Result<(), Box<dyn Error>> {
    let mut entries: HashSet<PathBuf> = HashSet::new();

    for entry in glob(path).unwrap().filter_map(Result::ok) {
        entries.insert(entry);
    }

    let result = json!({
      "code": 200,
      "success": true,
      "payload": {
          "list": entries
      }
    });

    println!("{}", result.to_string());
    Ok(())
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let p = "./*";
    inner_main(p).expect("Unable to process");
    Ok(cx.string("hello node"))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
