use jni_sys::*;

mod class;
mod environment;
mod method;
mod object;
mod string;
mod throwable;
mod value;

pub(crate) use self::class::*;
pub(crate) use self::environment::*;
pub(crate) use self::method::*;
pub(crate) use self::object::*;
pub(crate) use self::string::*;
pub(crate) use self::throwable::*;
pub(crate) use self::value::*;

#[derive(Clone, Debug)]
pub(crate) struct JVM {
  handle: *mut JavaVM
}

impl JVM {
  pub(crate) fn from_handle(handle: *mut JavaVM) -> JVM {
    return JVM { handle: handle };
  }
}
