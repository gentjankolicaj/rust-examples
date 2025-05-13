//export submodules outside persistence scope.
pub mod address;
pub mod address_dao;
pub mod user;
pub mod user_dao;

//import modules to be used on mod.rs scope
//directory/file-name/rust-item
use persistence::address::Address;
use persistence::address_dao::AddressDao;
use persistence::user::User;
use persistence::user_dao::UserDao;

pub fn create_users() {
    let a = user::User {
        id: 0,
        name: "john-doe".to_string(),
    };
    let b = user::User {
        id: 1,
        name: "jane-doe".to_string(),
    };

    println!("a={:?}", a);
    println!("b={:?}", b);
}

pub fn create_addresses() {
    let a = address::Address {
        zipcode: 1,
        street: "Street 1".to_string(),
    };
    let b = address::Address {
        zipcode: 2,
        street: "Street 2".to_string(),
    };

    println!("a={:?}", a);
    println!("b={:?}", b);
}

impl UserDao for User {
    fn print_all() {
        println!("UserDao implemented by User");
    }
}

impl AddressDao for Address {
    fn print_all() {
        println!("AddressDao implemented by Address");
    }
}
