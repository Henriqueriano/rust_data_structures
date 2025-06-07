#![allow(dead_code)]
struct Queue 
{
    head: i8,
    data: [i32;10]
}
trait QueueTrait 
{
    fn new() -> Queue;
    fn pop(&mut self) -> ();
    fn push(&mut self, val: i32) -> ();
    fn get_head(&mut self) -> i32;
    fn get_tail(&mut self) -> i32;
    fn is_empty(&mut self) -> bool;
    fn is_full(&mut self) -> bool;
    fn show_data(&mut self) -> ();
}
impl QueueTrait for Queue 
{
    fn new() -> Queue 
    {
        Queue 
        {
            head: -1,
            data: [0;10]
        }
    }
    fn pop(&mut self) -> () 
    {
        if Self::is_empty(self) 
        {
            return ();
        }
        self.head -= 1;
        ()
    }
    fn push(&mut self, val: i32) -> ()
    {
        if !Self::is_full(self) 
        {
            self.head += 1;
            self.data[self.head as usize] = val;
            return ();
        }
        panic!("Queue is full!");
    }
    fn get_head(&mut self) -> i32
    {
        if !Self::is_empty(self) 
        {
            return self.data[self.head as usize];
        }
        panic!("The queue is empty!");

    }
    fn get_tail(&mut self) -> i32 
    {
        1
    }
    fn is_empty(&mut self) -> bool
    {
        if self.head == -1
        {
            return true;
        }
        false
    }
    fn is_full(&mut self) -> bool
    {
        if self.head == 9 
        {
            return true;
        }
        false
    }
    fn show_data(&mut self) -> () 
    {
        println!("{:?}", self.data);
    }
}
fn main() 
{
    ()
}