use std;
use java;
use jvm;
use jvm::ToValue;

jvm_object!(Object, "java/lang/Object");

pub trait IObject {
  const CLASS_NAME: &'static str;

  fn from_jvm_object(object: jvm::Object) -> Self;
  fn as_jvm_object(&self) -> &jvm::Object;

  fn jvm_class() -> jvm::Class {
    return jvm::get_env().get_class(Self::CLASS_NAME).unwrap();
  }

  fn equals<T: IObject>(&self, other: &T) -> java::Result<bool> {
    return jvm_call!(bool: self, "equals", "(Ljava/lang/Object;)Z", &[&other.to_value()]);
  }

  // TODO: getClass()

  fn hash_code(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "hashCode", "()I", &[]);
  }

  // TODO: notify()
  // TODO: notifyAll()

  fn to_string(&self) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "toString", "()Ljava/lang/String;", &[]);
  }

  // TODO: wait()
  // TODO: wait(long timeout)
  // TODO: wait(long timeout, int nanos)
}

impl<'a, T: IObject> jvm::ToValue<'a> for &'a T {
  fn to_value(&self) -> jvm::Value<'a> {
    return self.as_jvm_object().to_value();
  }
}
