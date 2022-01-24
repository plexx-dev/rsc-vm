use core::num;
use std::path::Path;

mod vm;
mod vm_data;
mod opcodes;


fn main() {
    let given_args: Vec<String> = std::env::args().collect();
    let mut used_args = Vec::<f64>::with_capacity(given_args.len()-1);

    for i in 1..given_args.len() {
        let char : f64 = given_args[i].trim().parse().unwrap();
        used_args.push(char);
    }
    
    vm::run(Path::new("files/abc.rsbf"), used_args);
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
