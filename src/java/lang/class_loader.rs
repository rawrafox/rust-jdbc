use std;
use java;
use java::lang::Object;
use jvm;

pub struct ClassLoader(jvm::Object);

impl ClassLoader {
  pub(crate) fn from_jvm_object(object: jvm::Object) -> ClassLoader {
    return ClassLoader(object);
  }
  
  pub fn get_system_class_loader() -> java::Result<ClassLoader> {
    return jvm_call!(static, nonnull ClassLoader: "getSystemClassLoader", "()Ljava/lang/ClassLoader;", &[]);
  }
}

impl java::lang::Object for ClassLoader {
  const CLASS_NAME: &'static str = "java/lang/ClassLoader";

  fn from_jvm_object(object: jvm::Object) -> Self { return Self { 0: object }; }
  fn as_jvm_object(&self) -> &jvm::Object { return &self.0; }
}

impl std::fmt::Debug for ClassLoader {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.to_string() {
      Ok(None) => write!(f, "{} {{ null }}", Self::CLASS_NAME),
      Ok(Some(s)) => write!(f, "{} {{ {} }}", Self::CLASS_NAME, s),
      Err(_) => write!(f, "{} {{ ERROR }}", Self::CLASS_NAME)
    }
  }
}
