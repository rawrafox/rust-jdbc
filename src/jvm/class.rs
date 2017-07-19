use std;

use jvm::*;

#[derive(Debug)]
pub struct Class(jclass);

impl Class {
  pub(crate) fn from_handle(environment: Environment, handle: jclass) -> Class {
    return Class(environment.retain(handle));
  }

  pub fn find(name: &str) -> Result<Class, Object> {
    let environment = get_env();

    return environment.find_class(name);
  }

  pub fn get_method(&self, name: &str, signature: &str) -> Result<Method, Object> {
    let environment = get_env();
    let env = environment.as_handle();

    let name = std::ffi::CString::new(name).unwrap();
    let signature = std::ffi::CString::new(signature).unwrap();

    let handle = unsafe { (**env).GetMethodID.unwrap()(env, self.0, name.as_ptr(), signature.as_ptr()) };

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Method::from_handle(&environment, handle))
    }
  }

  pub fn get_static_method(&self, name: &str, signature: &str) -> Result<Method, Object> {
    let environment = get_env();
    let env = environment.as_handle();

    let name = std::ffi::CString::new(name).unwrap();
    let signature = std::ffi::CString::new(signature).unwrap();

    let handle = unsafe { (**env).GetStaticMethodID.unwrap()(env, self.0, name.as_ptr(), signature.as_ptr()) };

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Method::from_handle(&environment, handle))
    }
  }

  pub unsafe fn call_object_method(&self, method: &Method, arguments: &[&Value]) -> Result<Option<Object>, Object> {
    let environment = get_env();
    let env = environment.as_handle();

    let args : Vec<jvalue> = arguments.iter().map(|x| { x.as_handle() }).collect();

    let handle = (**env).CallStaticObjectMethodA.unwrap()(env, self.0, method.as_handle(), args.as_ptr());

    match environment.check_jvm_exception() {
      Some(e) => return Err(e),
      None => if handle.is_null() {
        return Ok(None);
      } else {
        return Ok(Some(Object::from_handle(&environment, handle)))
      }
    }
  }
}

impl Drop for Class {
  fn drop(&mut self) {
    let env = get_env().as_handle();

    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.0) };
  }
}
