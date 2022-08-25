#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate lazy_static;
extern crate core;

mod fetcher;
mod token;
mod pair;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
