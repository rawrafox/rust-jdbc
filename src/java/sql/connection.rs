use std;
use java;
use java::lang::IObject;
use java::sql::Statement;
use jvm;

jvm_object!(Connection, "java/sql/Connection");

impl Connection {  
  pub fn create_statement(&self) -> java::Result<Statement> {
    return jvm_call!(nonnull Statement: self, "createStatement", "()Ljava/sql/Statement;", &[]);
  }
}
