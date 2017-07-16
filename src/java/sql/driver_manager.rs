use std;
use java;
use java::lang::IObject;
use java::sql::Connection;
use jvm;
use jvm::ToValue;

jvm_object!(DriverManager, "java/sql/DriverManager");

impl DriverManager {  
  pub fn get_connection(url: &str) -> java::Result<Connection> {
    let url = jvm::String::from_str(url);

    return jvm_call!(static, nonnull Connection: "getConnection", "(Ljava/lang/String;)Ljava/sql/Connection;", &[&url.to_value()]);
  }
}
