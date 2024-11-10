use crate::utils::log;

use super::{DBConnection, Param};
use scylla::{Session, SessionBuilder};
use tokio::time::{sleep, Duration};

/// Type that represents a connection to a ScyllaDB database.
///
/// This type is a wrapper around the `scylla::Session` type.
/// It implements the `DBConnection` trait.
///
pub struct ScyllaConnection {
    pub connection: Session,
}

impl ScyllaConnection {
    pub async fn new(uri: &str) -> ScyllaConnection {
        loop {
            let session = SessionBuilder::new().known_node(uri).build().await;
            match session {
                Ok(session) => {
                    log::success("Connected to ScyllaDB 👽");
                    log::success(&format!("At {}", uri));
                    return ScyllaConnection {
                        connection: session,
                    };
                }
                Err(e) => {
                    log::error(&format!("Error connecting to ScyllaDB: {:?}", e));
                    log::info("↻ Retrying in 10 seconds... ⏲");
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }
    }
}

impl DBConnection for ScyllaConnection {
    fn close(self: Box<Self>) -> bool {
        true
    }

    fn execute(&self, _query: &str, _params: Vec<Param>) -> bool {
        todo!()
    }

    fn test(&self) {
        todo!()
    }
}
