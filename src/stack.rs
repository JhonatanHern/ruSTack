use std::mem;

#[derive(Debug)]
pub struct Stack <T> {
	size:u32,
	head:Option<Box<Node<T>>>,
}
#[derive(Debug)]
struct Node <T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}
impl <T> Stack <T>{
	pub fn new() -> Self {
        Stack{
        	size : 0,
        	head : None
        }
    }
    pub fn length(&self) -> u32{
    	self.size
    }
	pub fn push(&mut self,num: T){
		self.size = self.size + 1;
        let new_node = Box::new(Node {
            elem: num,
            next: mem::replace(&mut self.head, None),
        });
		self.head = Some(new_node);
	}
	pub fn pop(&mut self) -> Option<T>{
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