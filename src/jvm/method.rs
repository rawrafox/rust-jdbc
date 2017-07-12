use jvm::*;

#[derive(Clone, Debug)]
pub(crate) struct Method {
  environment: Environment,
  handle: jmethodID
}

impl Method {
  pub(crate) fn from_handle(environment: &Environment, handle: jmethodID) -> Method {
    return Method { environment: environment.clone(), handle: handle };
  }

  pub(crate) fn as_handle(&self) -> jmethodID {
    return self.handle;
  }
}
