use crate::helper::sum;
use crate::helper::string;

mod api;
mod core;
mod helper;

pub fn my_func() -> i32 {

    return sum(1, 5)
}

pub fn get_slice(s: &str) -> &str {
    string::slice(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
