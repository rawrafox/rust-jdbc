use std;
use java;
use java::ToValue;
use java::lang::IObject;
use jvm;

jvm_object!(Driver, "java/sql/Driver");

pub trait IDriver : IObject {
  fn accepts_url(&self, url: &str) -> java::Result<bool> {
    let url = jvm::String::from_str(url);

    return jvm_call!(bool: self, "acceptsURL", "(Ljava/lang/String;)Z", &[&url.to_value()]);
  }

  // Connection 	connect(String url, Properties info)

  fn get_major_version(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "getMajorVersion", "()I", &[]);
  }

  fn get_minor_version(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "getMinorVersion", "()I", &[]);
  }

  // Logger 	getParentLogger()
  // DriverPropertyInfo[] 	getPropertyInfo(String url, Properties info)

  fn jdbc_compliant(&self) -> java::Result<bool> {
    return jvm_call!(bool: self, "jdbcCompliant", "()Z", &[]);
  }
}

impl IDriver for Driver {}
