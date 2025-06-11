       // Contains user management logic (e.g., authentication, CRUD operations)

pub mod connection_pool;
pub use connection_pool::{get_db, init_db};