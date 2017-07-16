use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use jni_sys::*;

mod class;
mod environment;
mod method;
mod object;
mod string;
mod value;

pub(crate) use self::class::*;
pub(crate) use self::environment::*;
pub(crate) use self::method::*;
pub(crate) use self::object::*;
pub(crate) use self::string::*;
pub(crate) use self::value::*;

#[derive(Clone, Copy, Debug)]
pub struct JVM(*mut JavaVM);

static mut GLOBAL_JVM: Option<JVM> = None;

impl JVM {
  pub(crate) unsafe fn from_handle(handle: *mut JavaVM) -> JVM {
    let jvm = JVM(handle);

    if GLOBAL_JVM.is_none() {
      GLOBAL_JVM = Some(jvm);
    } else {
      panic!("A JVM is already initialised.")
    }

    return jvm;
  }

  pub unsafe fn from_options(options: &[&str]) -> (JVM, Environment) {
    let cstrings: Vec<CString> = options.iter().map(|x| CString::new(*x).unwrap()).collect();

    let mut jvm_options: Vec<JavaVMOption> = cstrings.iter().map(|x| {
      let mut jvm_option = JavaVMOption::default();
      jvm_option.optionString = x.as_ptr() as *mut i8;
      jvm_option
    }).collect();


    let mut jvm_arguments = JavaVMInitArgs::default();
    jvm_arguments.version = JNI_VERSION_1_8;
    jvm_arguments.options = jvm_options.as_mut_ptr();
    jvm_arguments.nOptions = jvm_options.len() as i32;
    jvm_arguments.ignoreUnrecognized = JNI_FALSE;

    let mut jvm: *mut JavaVM = ptr::null_mut();
    let mut env: *mut JNIEnv = ptr::null_mut();

    let result = JNI_CreateJavaVM(&mut jvm,
      (&mut env as *mut *mut JNIEnv) as *mut *mut c_void,
      (&mut jvm_arguments as *mut JavaVMInitArgs) as *mut c_void
    );

    if result != JNI_OK {
      let error_message = match result {
        JNI_EDETACHED => "thread detached from JVM",
        JNI_EEXIST => "JVM exists already",
        JNI_EINVAL => "invalid arguments",
        JNI_ENOMEM => "not enough memory",
        JNI_ERR => "unknown error",
        JNI_EVERSION => "JNI version error",
        _ => "unknown JNI error value",
      };

      panic!("Could not create JVM: {}", error_message);
    }

    return (JVM::from_handle(jvm), Environment::from_handle(env));
  }

  pub unsafe fn attach_thread(&self) -> Environment {
    let mut env: *mut JNIEnv = ptr::null_mut();

    let result = (**self.0).AttachCurrentThread.unwrap()(self.0,
      (&mut env as *mut *mut JNIEnv) as *mut *mut c_void,
      ptr::null_mut(),
    );

    if result != JNI_OK {
      let error_message = match result {
        JNI_EDETACHED => "thread detached from JVM",
        JNI_EEXIST => "JVM exists already",
        JNI_EINVAL => "invalid arguments",
        JNI_ENOMEM => "not enough memory",
        JNI_ERR => "unknown error",
        JNI_EVERSION => "JNI version error",
        _ => "unknown JNI error value",
      };

      panic!("Could not attach thread to JVM: {}", error_message);
    }

    return Environment::from_handle(env);
  }
  
  pub fn current_environment(&self) -> Option<Environment> {
    let mut env: *mut JNIEnv = ptr::null_mut();

    let result = unsafe {
      (**self.0).GetEnv.unwrap()(self.0, (&mut env as *mut *mut JNIEnv) as *mut *mut c_void, JNI_VERSION_1_8)
    };
    
    if result != JNI_OK {
      let error_message = match result {
        JNI_EDETACHED => "thread detached from JVM",
        JNI_EEXIST => "JVM exists already",
        JNI_EINVAL => "invalid arguments",
        JNI_ENOMEM => "not enough memory",
        JNI_ERR => "unknown error",
        JNI_EVERSION => "JNI version error",
        _ => "unknown JNI error value",
      };

      panic!("Could not get current JVM environment: {}", error_message);
    }

    if env.is_null() {
      return None;
    } else {
      return Some(Environment::from_handle(env));
    }
  }
}

pub fn global_jvm() -> Option<JVM> {
  return unsafe { GLOBAL_JVM };
}

pub fn current_environment() -> Option<Environment> {
  return global_jvm().and_then(|jvm| jvm.current_environment());
}

pub(crate) fn get_env() -> Environment {
  return current_environment().expect("Current thread was never attached to the JVM");
}