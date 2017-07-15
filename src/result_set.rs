use std;

use jvm::*;
use result_set_meta_data::*;

pub struct ResultSetObject {
  environment: Environment,
  class: Class,
  object: Object
}

impl ResultSetObject {
  pub(crate) fn new(environment: &Environment, object: Object) -> ResultSetObject {
    let class = environment.get_class("java/sql/ResultSet").unwrap();

    return ResultSetObject { environment: environment.clone(), class: class, object: object };
  }

  pub fn next(&self) -> Result<bool, Throwable> {
    let method = self.class.get_method("next", "()Z").unwrap();

    return unsafe { self.object.call_bool_method(&method, &[]) };
  }

  pub fn get_meta_data(&self) -> Result<ResultSetMetaDataObject, Throwable> {
    return java_call!(nonnull ResultSetMetaDataObject: self, "getMetaData", "()Ljava/sql/ResultSetMetaData;", &[]);
  }

  pub fn get_string(&self, index: i32) -> Result<Option<std::string::String>, Throwable> {
    return java_call!(string: self, "getString", "(I)Ljava/lang/String;", &[&index.to_value()]);
  }
}
