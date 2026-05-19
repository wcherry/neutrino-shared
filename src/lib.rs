pub mod api_error;
pub mod auth;
pub mod drive_client;
pub mod errors;
pub mod logger;
pub mod helper;
pub mod service_registry_client;

pub use api_error::ApiError;
pub use errors::{AppError, AppResult};
pub use logger::init_logging;
pub use helper::get_env_or_secret;
pub use service_registry_client::register_with_drive;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;