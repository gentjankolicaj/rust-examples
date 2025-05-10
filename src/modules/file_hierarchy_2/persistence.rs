//export submodules outside persistence scope.
pub mod dao;
pub mod model;

//import modules to be used on mod.rs scope
//directory/file-name/rust-item
use persistence::dao::address_dao::AddressDao;
use persistence::dao::user_dao::UserDao;
use persistence::model::address::Address;
use persistence::model::user::User;

pub fn create_users() {
    let a = User {
        id: 10,
        name: "john-doe1".to_string(),
    };
    let b = User {
        id: 11,
        name: "jane-doe1".to_string(),
    };

    println!("a={:?}", a);
    println!("b={:?}", b);
}

pub fn create_addresses() {
    let a = Address {
        zipcode: 11,
        street: "Street 11".to_string(),
    };
    let b = Address {
        zipcode: 12,
        street: "Street 12".to_string(),
    };

    println!("a={:?}", a);
    println!("b={:?}", b);
}

impl UserDao for User {
    fn find_all() -> Vec<User> {
        println!("UserDao implemented by User");

        return vec![User {
            id: 10,
            name: "john-doe".to_string(),
        }];
    }
}

impl AddressDao for Address {
    fn find_all() -> Vec<Address> {
        println!("AddressDao implemented by Address");

        return vec![Address {
            zipcode: 11,
            street: "Street 11".to_string(),
        }];
    }
}
