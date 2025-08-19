use which::which;
use std::path::PathBuf;

let cargo = which("cargo").unwrap();
assert_eq!(cargo, PathBuf::from("/usr/bin/cargo"));