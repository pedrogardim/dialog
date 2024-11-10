pub mod database;
pub mod utils;

use utils::log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::system("Welcome to Dialog 💬");

    #[cfg(feature = "mini")]
    log::system("Initializing in MINI mode 📦");

    #[cfg(feature = "standard")]
    log::system("Initializing in STANDARD mode 🚀");

    let conn = database::factory::create_db_connection("dialog.db").await?;
    conn.test();
    conn.close();
    Ok(())
}
