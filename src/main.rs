use std::path::Path;

mod vm;
mod vm_data;

fn main() {
    vm::run(Path::new("files/script.rsbf"));
}