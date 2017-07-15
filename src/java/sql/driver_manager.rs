use std;
use java;
use java::lang::Object;
use java::sql::Connection;
use jvm;
use jvm::ToValue;

pub struct DriverManager(jvm::Object);

impl DriverManager {  
  pub fn get_connection(url: &str) -> java::Result<Connection> {
    let url = jvm::String::from_str(url);

    return jvm_call!(static, nonnull Connection: "getConnection", "(Ljava/lang/String;)Ljava/sql/Connection;", &[&url.to_value()]);
  }
}

impl java::lang::Object for DriverManager {
  const CLASS_NAME: &'static str = "java/sql/DriverManager";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for DriverManager {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
