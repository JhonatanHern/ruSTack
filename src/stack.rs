use std::mem;

#[derive(Debug)]
pub struct Stack{
	size:i32,
	head:Option<Box<Node>>,
}
#[derive(Debug)]
pub struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}
impl Stack {
	pub fn new() -> Self {
        Stack{
        	size : 0,
        	head : None
        }
    }
	pub fn push(&mut self,num: i32){
		self.size = self.size + 1;
        let new_node = Box::new(Node {
            elem: num,
            next: mem::replace(&mut self.head, None),
        });
		self.head = Some(new_node);
	}
	pub fn pop(&mut self) -> Option<i32>{
		match mem::replace(&mut self.head,None) {
			Some(node) => {
				self.size = self.size - 1;
				self.head = (*node).next;
				Some((*node).elem)
			},
			None => None,
		}
	}
}