use std;
use java;
use java::lang::ClassLoader;
use java::lang::Object;
use jvm;
use jvm::ToValue;

pub struct Thread(jvm::Object);

impl Thread {  
  pub fn current_thread() -> java::Result<Thread> {
    return jvm_call!(static, nonnull Thread: "currentThread", "()Ljava/lang/Thread;", &[]);
  }

  pub fn set_context_class_loader(&self, class_loader: &ClassLoader) -> java::Result<()> {
    return jvm_call!(void: self, "setContextClassLoader", "(Ljava/lang/ClassLoader;)V", &[&class_loader.to_value()]);
  }
}

impl java::lang::Object for Thread {
  const CLASS_NAME: &'static str = "java/lang/Thread";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for Thread {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
