
pub mod stack;

// use first::Stack;

fn get_value(arg: Option<i32>) -> i32 {
	match arg {
		Some(num) => num,
		None => -1,
	}
}

#[cfg(test)]
mod tests {
	use crate::stack::Stack;
	use crate::get_value;
    #[test]
    fn push_n_pop() {
        let mut s = Stack::new();
        s.push(17);
        s.push(1);
        s.push(7);
        assert_eq!( 7 , get_value(s.pop()) );
        assert_eq!( 1 , get_value(s.pop()) );
        assert_eq!( 17 , get_value(s.pop()) );
        assert_eq!( None , s.pop() );
    }
}
