use std;
use java;
use java::lang::Object;
use java::sql::ResultSet;
use jvm;
use jvm::ToValue;

pub struct Statement(jvm::Object);

impl Statement {  
  pub fn execute(&self, sql: &str) -> java::Result<bool> {
    let sql = jvm::String::from_str(sql);

    return jvm_call!(bool: self, "execute", "(Ljava/lang/String;)Z", &[&sql.to_value()]);
  }
  
  pub fn execute_query(&self, sql: &str) -> java::Result<ResultSet> {
    let sql = jvm::String::from_str(sql);

    return jvm_call!(nonnull ResultSet: self, "executeQuery", "(Ljava/lang/String;)Ljava/sql/ResultSet;", &[&sql.to_value()]);
  }
}

impl java::lang::Object for Statement {
  const CLASS_NAME: &'static str = "java/sql/Statement";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for Statement {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
