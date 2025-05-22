//1.In rust-lang <T> declaration implies that T is generic type.
//2.In rust-lang generics can by bounded traits with declaration <T: TraitType>
//3.In rust-lang bounding generics with traits helps to stipulate behaviour of type.


use std::fmt::Debug;


//below is a struct with generic parameter & this generic is bounded to Debug trait
struct Val<T: Debug>{
    val: T

}

//this structure implements a method
impl<T:Debug> Val<T>{
    fn method_print(&self) {
        println!("val: {:?}", self.val);
    }
}



fn main() {
    let val_int=Val{val:3};
    let val_float=Val{val:3.0};

    //call method 'method_print()' of struct
    val_float.method_print();
    val_int.method_print();

}
