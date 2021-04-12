// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("Counting after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("Counting after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("Counting after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }
    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail());
}
