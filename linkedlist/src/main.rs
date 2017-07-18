enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn new() -> List{
        Nil 
    }

    fn prepend(self, val: i32) -> List{
        Cons(val, Box::new(self))
    }

    //fn delete(&self, val: i32){

    //}

    fn print(&self){
        match self{
            &Cons(val, next) => {
                print!("{} ", val);
                (*next).print();
            },
            &Nil => {

            }
        }
    }
}

fn main(){
    let list = List::new();
}
