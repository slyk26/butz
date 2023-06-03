mod users;
pub use users::User;

pub mod traits {
    use surrealdb::sql::Thing;

    pub trait Identifiable {
        fn get_id(&self) -> &Option<Thing>;
        fn set_id(&mut self, key: &str);
    }
}