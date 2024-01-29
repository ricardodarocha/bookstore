//pub use eyre::{bail,Result,WrapErr};

pub use crate::error::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;