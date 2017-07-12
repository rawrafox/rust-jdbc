use jvm::*;
use connection::*;

pub struct DriverManager {
  environment: Environment,
  class: Class
}

impl DriverManager {
  pub(crate) fn new(environment: &Environment) -> DriverManager {
    let class = environment.get_class("java/sql/DriverManager").unwrap();

    return DriverManager { environment: environment.clone(), class: class };
  }

  pub fn get_connection(&self, url: &str) -> Result<ConnectionObject, Throwable> {
    let method = self.class.get_static_method("getConnection", "(Ljava/lang/String;)Ljava/sql/Connection;").unwrap();

    let url = String::from_str(&self.environment, url);

    match unsafe { self.class.call_object_method(&method, &[&url.to_value()]) } {
      Ok(o) => return Ok(ConnectionObject::new(&self.environment, o)),
      Err(e) => return Err(e)
    }
  }
}
