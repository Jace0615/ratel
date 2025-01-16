pub mod assembly_members;
pub mod assets;
pub mod common_query_response;
pub mod error;
pub mod macros;
pub mod patrons;
pub mod topics;
pub mod users;

pub use assembly_members::*;
pub use assets::*;
pub use common_query_response::*;
pub use error::*;
pub use patrons::*;
pub use topics::*;
pub use users::*;

#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<error::ServiceError>>;

#[cfg(not(feature = "server"))]
pub type Result<T> = std::result::Result<T, error::ServiceError>;
