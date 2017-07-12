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
    let method = self.class.get_method("getMetaData", "()Ljava/sql/ResultSetMetaData;").unwrap();

    match unsafe { self.object.call_object_method(&method, &[]) } {
      Ok(o) => return Ok(ResultSetMetaDataObject::new(&self.environment, o.unwrap())),
      Err(e) => return Err(e)
    }
  }
  

  pub fn get_string(&self, index: i32) -> Result<Option<std::string::String>, Throwable> {
    let method = self.class.get_method("getString", "(I)Ljava/lang/String;").unwrap();

    match unsafe { self.object.call_object_method(&method, &[&index.to_value()]) } {
      Ok(o) => return Ok(o.map({ |h| unsafe { String::from_object(h) }.to_string() })),
      Err(e) => return Err(e)
    }
  }
}
