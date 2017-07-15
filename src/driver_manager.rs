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
    let url = String::from_str(&self.environment, url);

    return java_call!(static, nonnull ConnectionObject: self, "getConnection", "(Ljava/lang/String;)Ljava/sql/Connection;", &[&url.to_value()]);
  }
}
