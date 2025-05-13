//1.Slice are array whose length is not determined at compile time
//2.Slice is a 2 word object (1-st word is pointer to data , & 2-nd word is slice length).
//3.First word size is same as usize, usize bits depends on architecture.Ex. x86-64 arch => usize=>64
//4.Second word size is slice length
//5.Slice bnf : let identifier: &[type]=&array_identifier;
//6.Slices can be used to borrow sections of arrays.

fn print_slice_i32(slice: &[i32]) {
    println!("i32 slice length: {}, value: {:?}", slice.len(), slice);
}

fn print_slice_f32(slice: &[f32]) {
    println!("f32 slice length: {}, value: {:?}", slice.len(), slice);
}

fn main() {
    let i32_array: [i32; 5] = [1, 2, 3, 4, 5];
    //print 2 parts of array using slice
    print_slice_i32(&i32_array);
    print_slice_i32(&i32_array[0..2]);
    print_slice_i32(&i32_array[2..5]);

    let f32_array: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    //print 2 parts of array using slices
    print_slice_f32(&f32_array[0..6]);
    print_slice_f32(&f32_array[0..3]);
    print_slice_f32(&f32_array[3..5]);
}
