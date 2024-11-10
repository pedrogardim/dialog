pub mod scylla;
pub mod sqlite;

/// Trait that represents a connection to a database.
///
/// This trait is implemented by the `SQLiteConnection` and `ScyllaConnection` types.
/// - The `new` method creates a new connection to the database.
/// - The `close` method closes the connection to the database.
///
#[allow(async_fn_in_trait)]
pub trait DBConnection {
    async fn new(uri: &str) -> Self;
    fn close(self) -> bool;
    fn execute(&self, query: &str, params: Vec<Param>) -> bool;
    fn test(&self);
}

pub enum Param {
    Int(i32),
    Sting(String),
    Float(f64),
    Bool(bool),
}
