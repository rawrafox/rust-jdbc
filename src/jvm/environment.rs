use std::ffi::CString;

use jni_sys::*;

use jvm::*;

#[derive(Clone, Debug)]
pub(crate) struct Environment {
  jvm: JVM,
  handle: *mut JNIEnv
}

impl Environment {
  pub(crate) fn from_handle(jvm: JVM, handle: *mut JNIEnv) -> Environment {
    return Environment { jvm: jvm, handle: handle };
  }

  pub(crate) fn as_handle(&self) -> *mut JNIEnv {
    return self.handle;
  }

  pub(crate) fn retain(&self, handle: jobject) -> jobject {
    if handle.is_null() { panic!("Handle was null, could not retain") };

    let global = unsafe { (**self.handle).NewGlobalRef.unwrap()(self.handle, handle) };

    if global.is_null() { panic!("Retaining failed") };

    let _ = unsafe { (**self.handle).DeleteLocalRef.unwrap()(self.handle, handle) };

    return global;
  }

  pub fn get_class(&self, name: &str) -> Result<Class, Throwable> {
    let name = CString::new(name).unwrap();

    let handle = unsafe { (**self.handle).FindClass.unwrap()(self.handle, name.as_ptr()) };

    match self.check_jvm_exception() {
      Some(e) => return Err(e),
      None => return Ok(Class::from_handle(self.clone(), handle))
    }
  }

  pub fn check_jvm_exception(&self) -> Option<Throwable> {
    let exception = unsafe { (**self.handle).ExceptionOccurred.unwrap()(self.handle) };

    if !exception.is_null() {
      unsafe { (**self.handle).ExceptionDescribe.unwrap()(self.handle) };
      return Some(());
    }

    return None;
  }
}
