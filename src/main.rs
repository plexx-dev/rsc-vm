use std::path::Path;

mod vm;
mod vm_data;
mod opcodes;


fn main() {
    let given_args: Vec<String> = std::env::args().collect();
    let mut used_args = Vec::<f64>::with_capacity(given_args.len()-1);

    for i in 1..given_args.len() {
        match given_args[i].trim().parse::<f64>() {
            Ok(arg) => used_args.push(arg),
            Err(_) => {
                println!("Your Arguments were not correct");
                std::process::exit(1);
            },
        }
    }
    
    vm::run(Path::new("files/abc.rsbf"), used_args);
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::vm;

    #[test]
    fn test1() {
        vm::run(Path::new("files/abc.rsbf"), vec![10.0, 5.0]);
    }

    #[test]
    fn test2() {
        vm::run(Path::new("files/yo.rsbf"), vec![10.0, 5.0, 69.0]);
    }
}
