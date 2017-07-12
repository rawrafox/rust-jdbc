use jvm::*;
use statement::*;

pub struct ConnectionObject {
  environment: Environment,
  class: Class,
  object: Object
}

impl ConnectionObject {
  pub(crate) fn new(environment: &Environment, object: Object) -> ConnectionObject {
    let class = environment.get_class("java/sql/Connection").unwrap();

    return ConnectionObject { environment: environment.clone(), class: class, object: object };
  }

  pub fn create_statement(&self) -> Result<StatementObject, Throwable> {
    let method = self.class.get_method("createStatement", "()Ljava/sql/Statement;").unwrap();

    match unsafe { self.object.call_object_method(&method, &[]) } {
      Ok(o) => return Ok(StatementObject::new(&self.environment, o.unwrap())),
      Err(e) => return Err(e)
    }
  }
}
