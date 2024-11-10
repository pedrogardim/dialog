use std::error::Error;

#[cfg(feature = "standard")]
use crate::database::scylla::ScyllaConnection;
#[cfg(feature = "mini")]
use crate::database::sqlite::SQLiteConnection;

use crate::database::DBConnection;

pub async fn create_db_connection(uri: &str) -> Result<Box<dyn DBConnection>, Box<dyn Error>> {
    #[cfg(feature = "mini")]
    {
        return Ok(Box::new(SQLiteConnection::new(uri).await));
    }

    #[cfg(feature = "standard")]
    {
        return Ok(Box::new(ScyllaConnection::new(uri).await));
    }

    #[cfg(not(any(feature = "mini", feature = "standard")))]
    {
        Err("No feature enabled: Please enable either 'mini' or 'standard'".into())
    }
}
