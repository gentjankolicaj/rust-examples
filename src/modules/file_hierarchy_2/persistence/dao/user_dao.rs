use persistence::model::user::User;

pub trait UserDao {
    fn find_all() -> Vec<User>;
}
