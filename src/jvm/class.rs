use std;

use jvm::*;

#[derive(Debug)]
pub(crate) struct Class {
  environment: Environment,
  handle: jclass
}

impl Class {
  pub(crate) fn from_handle(environment: Environment, handle: jclass) -> Class {
    return Class { environment: environment.clone(), handle: environment.retain(handle) };
  }

  pub fn get_method(&self, name: &str, signature: &str) -> Result<Method, Throwable> {
    let env = self.environment.as_handle();

    let name = std::ffi::CString::new(name).unwrap();
    let signature = std::ffi::CString::new(signature).unwrap();

    let handle = unsafe { (**env).GetMethodID.unwrap()(env, self.handle, name.as_ptr(), signature.as_ptr()) };

    match self.environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Method::from_handle(&self.environment, handle))
    }
  }

  pub fn get_static_method(&self, name: &str, signature: &str) -> Result<Method, Throwable> {
    let env = self.environment.as_handle();

    let name = std::ffi::CString::new(name).unwrap();
    let signature = std::ffi::CString::new(signature).unwrap();

    let handle = unsafe { (**env).GetStaticMethodID.unwrap()(env, self.handle, name.as_ptr(), signature.as_ptr()) };

    match self.environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Method::from_handle(&self.environment, handle))
    }
  }

  pub unsafe fn call_object_method(&self, method: &Method, arguments: &[&Value]) -> Result<Object, Throwable> {
    let env = self.environment.as_handle();

    let args : Vec<jvalue> = arguments.iter().map(|x| { x.as_handle() }).collect();

    let handle = (**env).CallStaticObjectMethodA.unwrap()(env, self.handle, method.as_handle(), args.as_ptr());

    match self.environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Object::from_handle(&self.environment, handle))
    }
  }
}

impl Drop for Class {
  fn drop(&mut self) {
    let env = self.environment.as_handle();

    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.handle) };
  }
}
