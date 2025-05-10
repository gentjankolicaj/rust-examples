



    pub mod user_dao;
    pub mod address_dao;



pub fn create_users(){

    let fist=user::User{id:0,name:"john-doe".to_string()};
    let second=user::User{id:1,name:"jane-doe".to_string()};

}

pub fn create_addresses(){
    let fist=address::Address{zipcode:1,street:"Street 1".to_string()};
    let second=address::Address{zipcode:2,street:"Street 2".to_string()};
}