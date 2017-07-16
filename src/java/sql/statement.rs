use std;
use java;
use java::lang::IObject;
use java::sql::ResultSet;
use jvm;
use jvm::ToValue;

jvm_object!(Statement, "java/sql/Statement");

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
