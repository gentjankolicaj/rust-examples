//1.Usage of 'use' declaration so manual scoping is not needed
//2.We explicitly use each enum variant so they are available without manual scoping.
//3.Keyword 'use' BNF => use crate::enum_type{enum_variants...} or use crate::enum_type*
#[derive(Debug)]
enum Stage{
    Beginner,
    Advanced,
}

#[derive(Debug)]
enum Role{
    Student,
    Teacher,
}

fn main(){
    //Explicitly 'use' each enum variant so they are visible without manual scoping
    use crate::Stage::{Beginner, Advanced};
    use crate::Role::*; //use all enum variants of Role.


    //since I have 'use' above enum variants , enum variants a visible.
    let beginner = Beginner; //same as let beginner=Student::Beginner
    let student =Student;

    println!("Beginner: {:?}", beginner);
    println!("Student: {:?}", student);

}