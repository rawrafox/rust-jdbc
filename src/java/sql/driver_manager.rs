use std;
use java;
use java::ToValue;
use java::lang::IObject;
use java::sql::*;
use jvm;

jvm_object!(DriverManager, "java/sql/DriverManager");

impl DriverManager {
  pub fn get_connection(url: &str) -> java::Result<Connection> {
    let url = jvm::String::from_str(url);

    return jvm_call!(static, nonnull Connection: "getConnection", "(Ljava/lang/String;)Ljava/sql/Connection;", &[&url.to_value()]);
  }

  pub fn get_driver(url: &str) -> java::Result<Driver> {
    let url = jvm::String::from_str(url);

    return jvm_call!(static, nonnull Driver: "getDriver", "(Ljava/lang/String;)Ljava/sql/Driver;", &[&url.to_value()]);
  }
}
