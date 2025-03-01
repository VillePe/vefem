pub extern crate vputilslib;

pub mod fem;
pub mod loads;
pub mod structure;
pub mod material;
pub mod results;
pub mod settings;

pub fn add(left: u64, right: u64) -> u64 {
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
