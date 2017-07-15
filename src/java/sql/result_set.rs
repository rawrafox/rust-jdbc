use std;
use java;
use java::lang::Object;
use java::sql::ResultSetMetaData;
use jvm;
use jvm::ToValue;

pub struct ResultSet(jvm::Object);

impl ResultSet {  
  pub fn next(&self) -> java::Result<bool> {
    return jvm_call!(bool: self, "next", "()Z", &[]);
  }

  pub fn get_meta_data(&self) -> java::Result<ResultSetMetaData> {
    return jvm_call!(nonnull ResultSetMetaData: self, "getMetaData", "()Ljava/sql/ResultSetMetaData;", &[]);
  }

  pub fn get_string(&self, index: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getString", "(I)Ljava/lang/String;", &[&index.to_value()]);
  }
}

impl java::lang::Object for ResultSet {
  const CLASS_NAME: &'static str = "java/sql/ResultSet";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for ResultSet {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
