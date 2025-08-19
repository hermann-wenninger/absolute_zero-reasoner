use which::which;
use std::path::PathBuf;

let cargo_x = which("cargo").unwrap();
let rustc_x = which("rustc").unwrap();
let cargo = PathBuf::from(cargo_x);
let rustc = PathBuf::from(rustc_x);
