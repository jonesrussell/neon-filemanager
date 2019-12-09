extern crate neon;

#[macro_use]
extern crate serde_json;

extern crate glob;

use glob::glob;
use neon::prelude::*;
use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;

fn ls(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
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

    // println!("{}", result.to_string());
    Ok(result)
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
    let p = "./*";
    let value = ls(p).expect("Unable to process");
    let js_value = neon_serde::to_value(&mut cx, &value)?;
    Ok(js_value)
}

register_module!(mut cx, { cx.export_function("hello", hello) });
