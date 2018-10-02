enum List{
    Cons(i32,Rc<List>),
    Nil,
}

use std::rc::Rc;
use self::List::{Cons,Nil};

#[test]
fn test_rc(){
    let a = Rc::new(Cons(2,Rc::new(Cons(3,Rc::new(Nil)))));
    let b = Cons(2,Rc::clone(&a));
    assert_eq!(2,Rc::strong_count(&a));
}
