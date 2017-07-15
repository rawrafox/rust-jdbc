use std;
use java;
use java::lang::Object;
use jvm;

pub struct ResultSetMetaData(jvm::Object);

impl ResultSetMetaData {  
  pub fn get_column_count(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "getColumnCount", "()I", &[]);
  }
}

impl java::lang::Object for ResultSetMetaData {
  const CLASS_NAME: &'static str = "java/sql/ResultSetMetaData";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for ResultSetMetaData {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
