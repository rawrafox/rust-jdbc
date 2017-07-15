use jvm::*;
use result_set::*;

pub struct StatementObject {
  environment: Environment,
  class: Class,
  object: Object
}

impl StatementObject {
  pub(crate) fn new(environment: &Environment, object: Object) -> StatementObject {
    let class = environment.get_class("java/sql/Statement").unwrap();

    return StatementObject { environment: environment.clone(), class: class, object: object };
  }

  pub fn execute(&self, sql: &str) -> Result<bool, Throwable> {
    let sql = String::from_str(&self.environment, sql);

    return java_call!(bool: self, "execute", "(Ljava/lang/String;)Z", &[&sql.to_value()]);
  }
  
  pub fn execute_query(&self, sql: &str) -> Result<ResultSetObject, Throwable> {
    let sql = String::from_str(&self.environment, sql);

    return java_call!(nonnull ResultSetObject: self, "executeQuery", "(Ljava/lang/String;)Ljava/sql/ResultSet;", &[&sql.to_value()]);
  }
}
