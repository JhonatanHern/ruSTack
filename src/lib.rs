
pub mod stack;

// use first::Stack;

#[cfg(test)]
mod tests {
	use crate::stack::Stack;
    fn get_value_or_0(arg: Option<i32>) -> i32 {
        match arg {
            Some(num) => num,
            None => -1,
        }
    }
    fn get_value_or_empty_str(arg: Option<String>) -> String {
        match arg {
            Some(stri) => stri,
            None => String::from(""),
        }
    }
    #[test]
    fn push_n_pop_int() {
        let mut s = Stack::new();
        s.push( 17 );
        s.push( 1 );
        s.push( 7 );
        assert_eq!( 7 , get_value_or_0(s.pop()) );
        assert_eq!( 1 , get_value_or_0(s.pop()) );
        assert_eq!( 17 , get_value_or_0(s.pop()) );
        assert_eq!( None , s.pop() );
    }
    #[test]
    fn push_n_pop_str() {
        let mut s = Stack::new();
        s.push( String::from("17") );
        s.push( String::from("1") );
        s.push( String::from("7") );
        assert_eq!( String::from("7") , get_value_or_empty_str(s.pop()) );
        assert_eq!( String::from("1") , get_value_or_empty_str(s.pop()) );
        assert_eq!( String::from("17") , get_value_or_empty_str(s.pop()) );
        assert_eq!( None , s.pop() );
    }
}
