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
    let method = self.class.get_method("execute", "(Ljava/lang/String;)Z").unwrap();

    let sql = String::from_str(&self.environment, sql);

    return unsafe { self.object.call_bool_method(&method, &[&sql.to_value()]) };
  }
  
  pub fn execute_query(&self, sql: &str) -> Result<ResultSetObject, Throwable> {
    let method = self.class.get_method("executeQuery", "(Ljava/lang/String;)Ljava/sql/ResultSet;").unwrap();

    let sql = String::from_str(&self.environment, sql);

    match unsafe { self.object.call_object_method(&method, &[&sql.to_value()]) } {
      Ok(o) => return Ok(ResultSetObject::new(&self.environment, o.unwrap())),
      Err(e) => return Err(e)
    }
  }
}
