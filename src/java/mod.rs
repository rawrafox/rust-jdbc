use std;
use jvm;
use java::lang::IObject;

pub mod lang;
pub mod sql;

pub type Result<T> = std::result::Result<T, lang::Throwable>;

pub unsafe fn prepare_thread() -> Result<()> {
  let _ = jvm::global_jvm().unwrap().attach_thread();

  let current_thread = lang::Thread::current_thread()?;
  let class_loader = lang::ClassLoader::get_system_class_loader()?;
  current_thread.set_context_class_loader(&class_loader)?;

  return Ok(());
}

pub fn ensure_local_capacity(capacity: i32) -> Result<()> {
  match jvm::get_env().ensure_local_capacity(capacity) {
    Ok(()) => Ok(()),
    Err(e) => Err(lang::Throwable::from_jvm_object(e))
  }
}

pub trait ToValue<'a> {
  fn to_value(&self) -> jvm::Value<'a>;
}

impl<'a> ToValue<'a> for i32 {
  fn to_value(&self) -> jvm::Value<'a> {
    return jvm::Value::from_i32(*self);
  }
}

impl<'a> ToValue<'a> for i64 {
  fn to_value(&self) -> jvm::Value<'a> {
    return jvm::Value::from_i64(*self);
  }
}

impl<'a> ToValue<'a> for jvm::Object {
  fn to_value(&self) -> jvm::Value<'a> {
    return jvm::Value::from_object(self);
  }
}

impl<'a> ToValue<'a> for jvm::String {
  fn to_value(&self) -> jvm::Value<'a> {
    return jvm::Value::from_string(self);
  }
}
