use std;
use java;
use java::lang::IObject;
use java::sql::ResultSetMetaData;
use jvm;
use jvm::ToValue;

jvm_object!(ResultSet, "java/sql/ResultSet");

impl ResultSet {  
  pub fn next(&self) -> java::Result<bool> {
    return jvm_call!(bool: self, "next", "()Z", &[]);
  }

  pub fn get_meta_data(&self) -> java::Result<ResultSetMetaData> {
    return jvm_call!(nonnull ResultSetMetaData: self, "getMetaData", "()Ljava/sql/ResultSetMetaData;", &[]);
  }

  pub fn get_string(&self, index: i32) -> java::Result<Option<std::string::String>> {
    return jvm_call!(string: self, "getString", "(I)Ljava/lang/String;", &[&index.to_value()]);
  }
}
