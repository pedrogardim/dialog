pub mod database;
pub mod utils;

use database::DBConnection;
use utils::log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::system("Welcome to Dialog 💬");
    let conn = database::sqlite::SQLiteConnection::new("dialog.db").await;
    conn.test();
    conn.close();
    let conn = database::scylla::ScyllaConnection::new("dialog.db").await;
    conn.test();
    conn.close();
    Ok(())
}
