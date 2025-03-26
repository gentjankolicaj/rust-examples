use std::mem;

fn main(){
    let mut long_array:[i64;10] = [-100;10];
    println!("long_array: {:?}", long_array);

    //Update elements of array
    for i in 0..long_array.len(){
        long_array[i] = 100;
    }
    println!("updated long_array: {:?}", long_array);
    println!("Array long_array size {:?} bytes.",mem::size_of_val(&long_array));

}