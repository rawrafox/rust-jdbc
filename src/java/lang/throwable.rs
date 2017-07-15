use std;
use java;
use java::lang::Object;
use jvm;

pub struct Throwable(jvm::Object);

impl Throwable {
  pub fn get_message(&self) -> Result<Option<std::string::String>, Throwable> {
    return jvm_call!(string: self, "getMessage", "()Ljava/lang/String;", &[]);
  }
}

impl java::lang::Object for Throwable {
  const CLASS_NAME: &'static str = "java/lang/Throwable";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for Throwable {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
