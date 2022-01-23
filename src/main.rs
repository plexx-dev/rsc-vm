use std::path::Path;
use std::time::Instant;

mod vm;
mod vm_data;
mod opcodes;


fn main() {
    let start = Instant::now();
    vm::run(Path::new("files/abc.rsbf"), vec![420.0, 23.0]);
    let duration = start.elapsed();
    println!("{:?}", duration);
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