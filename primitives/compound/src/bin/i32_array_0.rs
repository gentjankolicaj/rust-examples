use std::mem;

fn main() {
    let mut i32_array: [i32; 10] = [1; 10];
    println!("i32_array: {:?}", i32_array);

    for i in 0..i32_array.len() {
        println!("initial-values: i32_array[{}]={}", i, i32_array[i]);
        i32_array[i] = 0;
    }

    for i in 0..i32_array.len() {
        println!("changed-values: i32_array[{}]={}", i, i32_array[i]);
    }

    for element in i32_array.iter().enumerate() {
        let (index, value): (usize, &i32) = element;
        println!("index={} value={}", index, value);
    }

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&i32_array));
}
