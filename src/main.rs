use std::path::Path;

mod vm;
mod vm_data;

fn main() {
    vm::run(Path::new("files/script.rsbf"));
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::vm;

    #[test]
    fn test() {
        vm::run(Path::new("files/script.rsbf"));
    }
}