//1.Slices are array whose length is not known at compile time.
//2.Slice is a 2 word object.
//3.The first word is a pointer to the data, the second word is the length of the slice
//4.The word size is same as usize. usize is determined by processor architecture.Eg. on arch x86-64 word size is 64 bit => usize=64 bit
//5.Slices can be used to borrow a section of an array and have the type signature &[T].

fn main() {
    let int_array: [i32; 10] = [1; 10];
    println!("int_array: {:?}", int_array);

    let left_slice: &[i32] =& int_array[0..5];
    let right_slice: &[i32] =& int_array[5..10];

    //simple iteration in slices
    print!("left-slice:");
    for el in left_slice {
        print!("{}", el);
    }
    print!("\nright-slice:");
    for el in right_slice {
        print!("{}", el);
    }

    //iter() iteration in slices
    print!("\nslice.iter() left-slice:");
    for element in left_slice.iter() {
        print!("{}", element);
    }
    print!("\nslice.iter() right-slice:");
    for element in right_slice.iter() {
        print!("{}", element);
    }
}
