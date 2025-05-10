

mod user{
    pub mod user;
    pub mod address;
}

mod dao{
    pub mod user_dao;
    pub mod address_dao;
}


pub fn create_users(){

    let fist=user::User{id:0,name:"john-doe".to_string()};
    let second=user::User{id:1,name:"jane-doe".to_string()};

}

pub fn create_addresses(){
    let fist=address::Addresse{zipcode:1,street:"Street 1".to_string()};
    let second=address::Addresse{zipcode:2,street:"Street 2".to_string()};
}