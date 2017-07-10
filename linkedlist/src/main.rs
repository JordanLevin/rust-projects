#[derive(Debug)]
struct Node {
    next: Node,
    val: i32,
    id: i32,
}

static count: i32 = 0;
impl Node{
    //prints the list
    fn print(&self){
        let current: &Node = &self;
        while current != None{
            println!("{:?}", current);
            current = current.next;
        }
    }

    //creates and adds a new node to the list
    fn add(&self, value: i32){
       let newNode = Node::new(value);
       let current: &Node = &self;
       while current.next != None{
           current = current.next;
       }
       current.next = newNode;
    }

    //Deleted node by id. returns false on fail and true on success
    fn del(&self, id: i32) -> bool{
        let current: &Node = &self;
        if current.id != id && current.next == None{
            return false;
        }
        let next: &Node = current.next;
        while next != None && next.id != id{
            current = next;
            next = next.next;
        }
        if next == None{
            return false;
        }
        current.next = next.next;
        return true;
    }
    fn new(value: i32) -> Node{
        count = count+1;
        return Node {next: None, val: value, id: count}; 
    }
}

fn main() {
    println!("Hello, world!");
}
