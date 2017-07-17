use std;

use jni_sys::*;

use jvm::*;

#[derive(Clone)]
pub struct Value<'a> {
  handle: jvalue,
  _data: std::marker::PhantomData<&'a jvalue>
}

impl<'a> Value<'a> {
  pub(crate) fn from_handle(handle: jvalue) -> Value<'a> {
    return Value { handle: handle, _data: std::marker::PhantomData };
  }

  pub(crate) fn as_handle(&self) -> jvalue {
    return self.handle;
  }
}

pub(crate) trait ToValue<'a> {
  fn to_value(&self) -> Value<'a>;
}

impl<'a> ToValue<'a> for i32 {
  fn to_value(&self) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.i() = *self };

    return Value::from_handle(handle);
  }
}

impl<'a> ToValue<'a> for i64 {
  fn to_value(&self) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.j() = *self };

    return Value::from_handle(handle);
  }
}

impl<'a> ToValue<'a> for Object {
  fn to_value(&self) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.l() = self.as_handle() };

    return Value::from_handle(handle);
  }
}

impl<'a> ToValue<'a> for String {
  fn to_value(&self) -> Value<'a> {
    let mut handle = jvalue::default();

    unsafe { *handle.l() = self.as_handle() };

    return Value::from_handle(handle);
  }
}
