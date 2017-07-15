use std;

pub mod lang;
pub mod sql;

pub type Result<T> = std::result::Result<T, lang::Throwable>;