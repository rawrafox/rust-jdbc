use std;
use java;
use java::lang::IObject;
use jvm;

jvm_object!(Throwable, "java/lang/Throwable");

impl Throwable {
  pub fn get_message(&self) -> Result<Option<std::string::String>, Throwable> {
    return jvm_call!(string: self, "getMessage", "()Ljava/lang/String;", &[]);
  }
}
