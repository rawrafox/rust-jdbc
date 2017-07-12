use std;

use jvm::*;

#[derive(Debug)]
pub(crate) struct String {
  environment: Environment,
  handle: jstring
}

impl String {
  pub(crate) unsafe fn from_object(object: Object) -> String {
    return std::mem::transmute(object);
  }

  pub(crate) fn from_handle(environment: &Environment, handle: jstring) -> String {
    return String { environment: environment.clone(), handle: environment.retain(handle) };
  }

  pub(crate) fn as_handle(&self) -> jstring {
    return self.handle;
  }

  pub fn from_str(environment: &Environment, string: &str) -> String {
    let env = environment.as_handle();

    let string = std::ffi::CString::new(string).unwrap();

    let handle = unsafe { (**env).NewStringUTF.unwrap()(env, string.as_ptr()) };

    return String::from_handle(environment, handle)
  }

  pub fn to_string(&self) -> std::string::String {
    let env = self.environment.as_handle();

    return unsafe {
      let ptr = (**env).GetStringUTFChars.unwrap()(env, self.handle, std::ptr::null_mut());

      let string = std::ffi::CStr::from_ptr(ptr).to_str().unwrap().to_string();

      (**env).ReleaseStringUTFChars.unwrap()(env, self.handle, ptr);

      string
    };
  }
}

impl Drop for String {
  fn drop(&mut self) {
    let env = self.environment.as_handle();

    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.handle) };
  }
}
