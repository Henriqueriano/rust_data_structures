#![allow(dead_code)]
pub struct Stack {
    head: i8,
    data: [i32; 10],
}
impl Stack {
    pub fn new() -> Self {
        Stack {
            head: -1,
            data: [0; 10],
        }
    }
    pub fn pop(&mut self) -> () {
        if Self::is_empty(self) {
            panic!("Cannot remove, the stack is empty!");
        }
        self.head -= 1;
    }
    pub fn push(&mut self, val: i32) -> () {
        if Self::is_full(self) {
            panic!("Stack is full, cannot insert!");
        }
        self.head += 1;
        self.data[self.head as usize] = val;
    }
    pub fn show_data(&self) -> () {
        println!("{:?}", self.data);
    }
    fn is_empty(&self) -> bool {
        if self.head == -1 {
            return true;
        }
        false
    }
    fn is_full(&self) -> bool {
        if self.head == 9 {
            return true;
        }
        false
    }
}
