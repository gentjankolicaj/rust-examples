//1.In rust in order to allow conversion of type T to String we must implement 'ToString' trait or 'fmt::Display' trait which automatically provides ToString


//declare c_struct
struct Person{
    id:i64,
}

impl ToString for Person{
    fn to_string(&self) -> String{
        return  format!("id:{}", self.id)
    }
}


fn main(){
    //Instantiate Person
    let p0 = Person{id:0};
    let p1 = Person{id:1};

    println!("c-struct p0={},p1={}", p0.to_string(), p1.to_string());
}