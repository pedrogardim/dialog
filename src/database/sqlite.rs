use std::vec;

use crate::utils::log;

use super::{DBConnection, Param};
use rusqlite::{Connection, ToSql};

/// Type that represents a connection to a SQLite database.
///
/// This type is a wrapper around the `rusqlite::Connection` type.
/// It implements the `DBConnection` trait.
///
pub struct SQLiteConnection {
    pub connection: Connection,
}

impl Param {
    fn as_tosql(&self) -> &dyn ToSql {
        match self {
            Param::Int(i) => i,
            Param::Sting(s) => s,
            Param::Float(f) => f,
            Param::Bool(b) => b,
        }
    }
}

impl DBConnection for SQLiteConnection {
    async fn new(path: &str) -> SQLiteConnection {
        log::info(&format!("Connecting to SQLite 🪶  at {}...", path));
        let conn = Connection::open(path).unwrap();
        log::success("Connected to SQLite 🪶");
        SQLiteConnection { connection: conn }
    }

    fn close(self) -> bool {
        match self.connection.close() {
            Ok(_) => {
                log::success("Closed connection to SQLite");
                true
            }
            Err(e) => {
                log::error(&format!("Error closing connection: {:?}", e));
                false
            }
        }
    }

    fn execute(&self, query: &str, params: Vec<Param>) -> bool {
        let params_tosql: Vec<&dyn ToSql> = params.iter().map(|param| param.as_tosql()).collect();
        match self.connection.execute(query, &params_tosql[..]) {
            Ok(_) => true,
            Err(e) => {
                log::error(&format!("Error executing query: {:?}", e));
                false
            }
        }
    }

    fn test(&self) {
        struct User {
            name: String,
            age: u32,
        }

        let vector = vec![Param::Sting("Alice".to_string()), Param::Int(42)];

        self.execute(
            "CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER)",
            vec![],
        );

        self.execute("INSERT INTO users (name, age) VALUES (?1, ?2)", vector);

        let mut stmt = self
            .connection
            .prepare("SELECT name, age FROM users")
            .unwrap();

        let user = stmt
            .query_row([], |row| {
                Ok(User {
                    name: row.get(0)?,
                    age: row.get(1)?,
                })
            })
            .unwrap();

        log::debug(&format!("User: {} - {}", user.name, user.age))
    }
}
