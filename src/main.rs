use std::path::Path;

mod vm;
mod vm_data;
mod opcodes;


fn main() {
    
    vm::run(Path::new("files/abc.rsbf"), vec![420.0, 23.0]);
    
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