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
    return java_call!(nonnull StatementObject: self, "createStatement", "()Ljava/sql/Statement;", &[]);
  }
}
