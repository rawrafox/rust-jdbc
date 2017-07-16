use std;
use java;
use java::lang::IObject;
use jvm;

jvm_object!(ResultSetMetaData, "java/sql/ResultSetMetaData");

impl ResultSetMetaData {  
  pub fn get_column_count(&self) -> java::Result<i32> {
    return jvm_call!(int: self, "getColumnCount", "()I", &[]);
  }
}
