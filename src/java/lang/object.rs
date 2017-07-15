use std;
use java;
use jvm;

pub trait Object {
  const CLASS_NAME: &'static str;

  fn from_jvm_object(object: jvm::Object) -> Self;
  fn as_jvm_object(&self) -> &jvm::Object;

  fn jvm_class() -> jvm::Class {
    return jvm::current_environment().unwrap().get_class(Self::CLASS_NAME).unwrap();
  }

  fn to_string(&self) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "toString", "()Ljava/lang/String;", &[]);
  }
}

impl<'a, T: Object> jvm::ToValue<'a> for &'a T {
  fn to_value(&self) -> jvm::Value<'a> {
    return self.as_jvm_object().to_value();
  }
}
