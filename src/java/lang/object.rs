use std;
use java;
use jvm;
use jvm::ToValue;

jvm_object!(Object, "java/lang/Object");

pub trait IObject : Sized {
  const CLASS_NAME: &'static str;

  fn from_jvm_object(object: jvm::Object) -> Self;
  fn as_jvm_object(&self) -> &jvm::Object;

  fn jvm_class() -> jvm::Class {
    return jvm::get_env().get_class(Self::CLASS_NAME).unwrap();
  }

  fn synchronize<'a>(&'a self) -> MonitorGuard<'a, Self> {
    unsafe { self.as_jvm_object().enter_monitor() };

    return MonitorGuard(&self);
  }

  fn equals<T: IObject>(&self, other: &T) -> java::Result<bool> {
    return jvm_call!(bool: self, "equals", "(Ljava/lang/Object;)Z", &[&other.to_value()]);
  }

  // TODO: getClass()

  fn hash_code(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "hashCode", "()I", &[]);
  }

  fn to_string(&self) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "toString", "()Ljava/lang/String;", &[]);
  }
}

impl<'a, T: IObject> jvm::ToValue<'a> for &'a T {
  fn to_value(&self) -> jvm::Value<'a> {
    return self.as_jvm_object().to_value();
  }
}

pub struct MonitorGuard<'a, T: 'a + IObject>(&'a T);

impl<'a, T: IObject> MonitorGuard<'a, T> {
  pub fn notify(&self) -> java::Result<()> {
    let method = T::jvm_class().get_method("notify", "()V").unwrap();

    match unsafe { self.0.as_jvm_object().call_void_method(&method, &[]) } {
      Ok(()) => Ok(()),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  }

  pub fn notify_all(&self) -> java::Result<()> {
    let method = T::jvm_class().get_method("notifyAll", "()V").unwrap();

    match unsafe { self.0.as_jvm_object().call_void_method(&method, &[]) } {
      Ok(()) => Ok(()),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  }

  pub fn wait(&self) -> java::Result<()> {
    let method = T::jvm_class().get_method("wait", "()V").unwrap();

    match unsafe { self.0.as_jvm_object().call_void_method(&method, &[]) } {
      Ok(()) => Ok(()),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  }

  pub fn wait_seconds(&self, seconds: i64) -> java::Result<()> {
    let method = T::jvm_class().get_method("wait", "(J)V").unwrap();

    match unsafe { self.0.as_jvm_object().call_void_method(&method, &[&seconds.to_value()]) } {
      Ok(()) => Ok(()),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  }

  pub fn wait_seconds_nanos(&self, seconds: i64, nanos: i32) -> java::Result<()> {
    let method = T::jvm_class().get_method("wait", "(JI)V").unwrap();

    match unsafe { self.0.as_jvm_object().call_void_method(&method, &[&seconds.to_value(), &nanos.to_value()]) } {
      Ok(()) => Ok(()),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  }

  pub fn exit(self) {}
}

impl<'a, T: IObject> Drop for MonitorGuard<'a, T> {
  fn drop(&mut self) {
    unsafe { self.0.as_jvm_object().exit_monitor() };
  }
}
