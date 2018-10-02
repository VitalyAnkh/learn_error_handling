extern crate learn_error_handling;



fn it_add_two(){
    assert_eq!(learn_error_handling::add_two(3),5);
}

fn check_borrow_rules(){
    let a = 8;
    let y =&a;
}