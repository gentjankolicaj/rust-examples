use persistence::model::address::Address;

pub trait AddressDao {
    fn find_all() -> Vec<Address>;
}
