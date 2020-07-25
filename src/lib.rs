pub fn this_function(x: i8) -> i8 {
    x + 5
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn fail() {
        assert_eq!(4, 5);
    }

    #[test]
    fn this_function_0() {
        assert_eq!(this_function(4), 9);
    }
}
