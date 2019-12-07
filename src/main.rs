#[macro_use]
extern crate serde_json;

use glob::glob;
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

fn main() {
  let p = "./*";
  inner_main(p).expect("Unable to process");
}
