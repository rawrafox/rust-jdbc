use jvm::*;

pub struct ResultSetMetaDataObject {
  environment: Environment,
  class: Class,
  object: Object
}

impl ResultSetMetaDataObject {
  pub(crate) fn new(environment: &Environment, object: Object) -> ResultSetMetaDataObject {
    let class = environment.get_class("java/sql/ResultSetMetaData").unwrap();

    return ResultSetMetaDataObject { environment: environment.clone(), class: class, object: object };
  }

  pub fn get_column_count(&self) -> Result<i32, Throwable> {
    let method = self.class.get_method("getColumnCount", "()I").unwrap();

    return unsafe { self.object.call_int_method(&method, &[]) };
  }
}
