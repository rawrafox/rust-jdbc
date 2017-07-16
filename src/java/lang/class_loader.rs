use std;
use java;
use java::lang::IObject;
use jvm;

jvm_object!(ClassLoader, "java/lang/ClassLoader");

impl ClassLoader {
  pub fn get_system_class_loader() -> java::Result<ClassLoader> {
    return jvm_call!(static, nonnull ClassLoader: "getSystemClassLoader", "()Ljava/lang/ClassLoader;", &[]);
  }
}
