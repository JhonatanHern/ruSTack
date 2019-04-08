
pub mod stack;

#[cfg(test)]
mod tests {
	use crate::stack::Stack;
    #[test]
    fn push_n_pop_int() {
        let mut s = Stack::new();
        assert_eq!( 0 , s.length() );
        s.push( 17 );
        s.push( 1 );
        s.push( 7 );
        assert_eq!( 3 , s.length() );
        assert_eq!( 7 , s.pop().unwrap() );
        assert_eq!( 1 , s.pop().unwrap() );
        assert_eq!( 17 , s.pop().unwrap() );
        assert_eq!( 0 , s.length() );
        assert_eq!( None , s.pop() );
        assert_eq!( 0 , s.length() );
    }
    #[test]
    fn push_n_pop_str() {
        let mut s = Stack::new();
        assert_eq!( 0 , s.length() );
        s.push( String::from("17") );
        s.push( String::from("1") );
        s.push( String::from("7") );
        assert_eq!( 3 , s.length() );
        assert_eq!( String::from("7") , s.pop().unwrap() );
        assert_eq!( String::from("1") , s.pop().unwrap() );
        assert_eq!( String::from("17") , s.pop().unwrap() );
        assert_eq!( 0 , s.length() );
        assert_eq!( None , s.pop() );
        assert_eq!( 0 , s.length() );
    }
}
