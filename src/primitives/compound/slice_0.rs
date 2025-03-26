//1.Slices are array whose length is not known at compile time.
//2.Slice is a 2 word object.
//3.The first word is a pointer to the data, the second word is the length of the slice
//4.The word size is same as usize. usize is determined by processor architecture.Eg. on arch x86-64 word size is 64 bit => usize=64 bit
//5.Slices can be used to borrow a section of an array and have the type signature &[T].

fn print_slice_info(arg: &[i32]) {
    println!("\nfn print_slice_info()");
    println!("slice length {:?} values: {:?}", arg.len(), arg);
    println!("slice 0 index: {:?}", arg[0]);
}
fn main() {
    let int_array: [i32; 10] = [1; 10];
    println!("int_array: {:?}", int_array);

    //call function to print slice details
    print_slice_info(&int_array);

    //slices can point to a section of array
    let partial_slice = &int_array[3..7];
    print_slice_info(partial_slice);
}
