//1.A tuple is a collection of values of different types.
//2.Tuples are constructed using parenthesis.
//3.Each tuple itself is a value with type signature (T1,T2,T3...) where T1,T2 are types of its members.
//4.Functions can use tuples to return values as tuples hold any number of values.

fn main() {
    let one_element_tuple = (10i8,);
    println!("one_element_tuple= {:?}", one_element_tuple);

    println!("implicit one_element_tuple= {:?}", (101));
}
