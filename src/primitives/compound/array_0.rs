//1.Array is a collection of same objects of type T
//2.Arrays are stored in contiguous memory
//3.Arrays are declare with brackets []
//4.Array length is known at compile time and is part of type signature.Ex let identifier:[T,length]

fn main() {
    let f32_array: [f32; 10] = [0.0; 10];
    println!("f32_array: {:?}", f32_array);

    let i64_array: [i64; 10] = [0; 10];
    println!("i64_array: {:?}", i64_array);

    //declaration & initialization with repeated copy
    let bool_array: [bool; 3] = [true; 3]; //array is expanded and true copied 3 times.
    println!("bool_array {:?}", bool_array);

    let double_array: [f64; 5] = [1.0; 5];
    println!("double_array {:?}", double_array);

    let mut bool_array: [bool; 10] = [false; 10];
    println!("mut bool_array: {:?}", bool_array);

    //update array at index 0
    bool_array[0] = true;
    println!("mut bool_array: {:?}", bool_array);
}
