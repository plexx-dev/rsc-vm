use std::path::Path;
use std::env;

mod vm;
mod vm_data;
mod opcodes;


fn main() {
    let args: Vec<String> = env::args().collect();
    vm::run(Path::new("files/shit.rsbf"), vec![420.0, 69.0]);
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::vm;

    #[test]
    fn test() {
        vm::run(Path::new("files/script.rsbf"), vec![10.0, 5.0]);
    }
}