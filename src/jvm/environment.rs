use std::ffi::CString;

use jni_sys::*;

use jvm::*;

#[derive(Clone, Debug)]
pub struct Environment(*mut JNIEnv);

impl Environment {
  pub(crate) fn from_handle(handle: *mut JNIEnv) -> Environment {
    return Environment(handle);
  }

  pub(crate) fn as_handle(&self) -> *mut JNIEnv {
    return self.0;
  }

  pub(crate) fn retain(&self, handle: jobject) -> jobject {
    if handle.is_null() { panic!("Handle was null, could not retain") };

    let global = unsafe { (**self.0).NewGlobalRef.unwrap()(self.0, handle) };

    if global.is_null() { panic!("Retaining failed") };

    let _ = unsafe { (**self.0).DeleteLocalRef.unwrap()(self.0, handle) };

    return global;
  }

  pub fn get_class(&self, name: &str) -> Result<Class, Object> {
    let name = CString::new(name).unwrap();

    let handle = unsafe { (**self.0).FindClass.unwrap()(self.0, name.as_ptr()) };

    match self.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Class::from_handle(self.clone(), handle))
    }
  }

  pub fn check_jvm_exception(&self) -> Option<Object> {
    let exception = unsafe { (**self.0).ExceptionOccurred.unwrap()(self.0) };

    if !exception.is_null() {
      unsafe { (**self.0).ExceptionClear.unwrap()(self.0) };

      return Some(Object::from_handle(self, exception));
    }

    return None;
  }
}
