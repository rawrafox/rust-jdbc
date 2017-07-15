use std;
use java;
use java::lang::Object;
use java::sql::Statement;
use jvm;

pub struct Connection(jvm::Object);

impl Connection {  
  pub fn create_statement(&self) -> java::Result<Statement> {
    return jvm_call!(nonnull Statement: self, "createStatement", "()Ljava/sql/Statement;", &[]);
  }
}

impl java::lang::Object for Connection {
  const CLASS_NAME: &'static str = "java/sql/Connection";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for Connection {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
