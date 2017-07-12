use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use jni_sys::*;

use jvm::*;
use driver_manager::*;

#[derive(Clone, Debug)]
pub struct Context {
  jvm: JVM,
  environment: Environment
}

impl Context {
  pub unsafe fn from_handle(jvm: *mut JavaVM, environment: *mut JNIEnv) -> Context {
    let jvm = JVM::from_handle(jvm);
    let environment = Environment::from_handle(jvm.clone(), environment);

    return Context {
      jvm: jvm,
      environment: environment
    };
  }

  pub unsafe fn from_options(options: &[&str]) -> Context {
    let cstrings : Vec<CString> = options.iter().map(|x| CString::new(*x).unwrap()).collect();

    let mut jvm_options : Vec<JavaVMOption> = cstrings.iter().map(|x| {
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
    let mut jni_environment : *mut JNIEnv = ptr::null_mut();

    let result = JNI_CreateJavaVM(&mut jvm,
      (&mut jni_environment as *mut *mut JNIEnv) as *mut *mut c_void,
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

      panic!("`JNI_CreateJavaVM()` signaled an error: {}", error_message);
    }

    return Context::from_handle(jvm, jni_environment);
  }

  pub fn driver_manager(&self) -> DriverManager {
    return DriverManager::new(&self.environment);
  }
}
