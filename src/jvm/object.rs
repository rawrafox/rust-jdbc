use jvm::*;

#[derive(Debug)]
pub(crate) struct Object {
  environment: Environment,
  handle: jobject
}

impl Object {
  pub(crate) fn from_handle(environment: &Environment, handle: jobject) -> Object {
    return Object { environment: environment.clone(), handle: environment.retain(handle) };
  }

  pub(crate) fn as_handle(&self) -> jobject {
    return self.handle;
  }
  
  pub unsafe fn call_bool_method(&self, method: &Method, arguments: &[&Value]) -> Result<bool, Throwable> {
    let env = self.environment.as_handle();

    let args : Vec<jvalue> = arguments.iter().map(|x| { x.as_handle() }).collect();

    let result = (**env).CallBooleanMethodA.unwrap()(env, self.handle, method.as_handle(), args.as_ptr());

    match self.environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result != 0)
    }
  }

  pub unsafe fn call_int_method(&self, method: &Method, arguments: &[&Value]) -> Result<i32, Throwable> {
    let env = self.environment.as_handle();

    let args : Vec<jvalue> = arguments.iter().map(|x| { x.as_handle() }).collect();

    let result = (**env).CallIntMethodA.unwrap()(env, self.handle, method.as_handle(), args.as_ptr());

    match self.environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(result)
    }
  }

  pub unsafe fn call_object_method(&self, method: &Method, arguments: &[&Value]) -> Result<Option<Object>, Throwable> {
    let env = self.environment.as_handle();

    let args : Vec<jvalue> = arguments.iter().map(|x| { x.as_handle() }).collect();

    let handle = (**env).CallObjectMethodA.unwrap()(env, self.handle, method.as_handle(), args.as_ptr());

    match self.environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => if handle.is_null() {
        return Ok(None);
      } else {
        return Ok(Some(Object::from_handle(&self.environment, handle)))
      }
    }
  }
}

impl Drop for Object {
  fn drop(&mut self) {
    let env = self.environment.as_handle();

    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.handle) };
  }
}
