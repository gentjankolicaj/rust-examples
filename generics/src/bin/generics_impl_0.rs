//1.In rust-lang <T> declaration implies that T is generic type.

#![allow(dead_code)]
#![allow(unused_variables)]

struct Point<T> {
    x: T,
    y: T,
}


//concrete implementation for f32
impl Point<f32> {
    fn info_float(&self) {
        println!("x = {}, y = {}", self.x, self.y);
    }
}

//generic implementation
impl<T: std::fmt::Debug> Point<T> {
    fn info_generic(&self)  {
        println!("x = {:?}, y = {:?}", self.x, self.y);
    }
}


fn main(){
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    
    //call 2 methods of both_float.
    both_float.info_float();
    both_float.info_generic();
    
    //call method of both_integers
    both_integer.info_generic();
    // both_integer.info_float(); //compile since info_float is implemented only for concrete type f42
}

