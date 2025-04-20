use std::{collections::BTreeMap, ffi::OsStr, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::class::Class;

#[derive(Default, Serialize, Deserialize)]
pub struct Index(BTreeMap<String, Class>);

impl Index {
    pub fn populate(&mut self, path: &Path) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();

            if entry.metadata().unwrap().is_dir() {
                self.populate(entry.path().as_path());
                continue;
            }

            if entry.path().extension() != Some(OsStr::new("java")) {
                continue;
            }

            let class = Class::from_str(&fs::read_to_string(entry.path()).unwrap());

            self.0.insert(class.qualified_name.clone(), class);
        }
    }
}
