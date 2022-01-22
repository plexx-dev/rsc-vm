use std::path::Path;

use crate::vm_data::Data;

pub fn run(file_path: &Path) {
    let data = Data::new(file_path);
}