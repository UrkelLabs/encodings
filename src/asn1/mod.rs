mod bit_string;
mod error;
mod object_identifier;
mod parser;
mod traits;

use bit_string::BitString;
pub use error::ParseError;
use object_identifier::ObjectIdentifier;

pub type Result<T> = std::result::Result<T, error::ParseError>;
