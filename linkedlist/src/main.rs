enum Node {
    Cons(i32, Box<List>),
    Nil,
}

use Node::{Cons, Nil};

struct List {
    pub head: Node,
}

impl List -> List{
    fn new(){
        List{Node: Nil} 
    }

    fn add(mut &self, val: i32){
        let &mut current: Node = &mut self.head;
        let mut next: Node = Nil;
        match current {
            Nil => {
                current
            },
            Cons(num, next) => {
                next = current.next;
            },
        }
        loop {
            match current{
                Cons(num, next) =>{

                },
                Nil => {
                    
                },
            }
        }
    }

    fn delete(&self, val: i32){

    }

    fn print(&self){

    }
}

fn main(){
    let list = List::Cons(1, Box::new(List::Cons(5, Box::new(List::Nil))));
}
