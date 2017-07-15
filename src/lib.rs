// Copyright 2017 The rust-jdbc developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(associated_consts)]

extern crate jni_sys;

#[link(name="jvm")]
extern { }

macro_rules! jvm_call {
  (static, nonnull $rty:tt: $name:expr, $sig:expr, $args:expr) => ({
    let class = Self::jvm_class();
    let method = class.get_static_method($name, $sig).unwrap();

    match unsafe { class.call_object_method(&method, $args) } {
      Ok(o) => Ok($rty::from_jvm_object(o.unwrap())),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  });
  (void: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = Self::jvm_class().get_method($name, $sig).unwrap();

    match unsafe { $s.as_jvm_object().call_void_method(&method, $args) } {
      Ok(()) => Ok(()),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  });
  (bool: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = Self::jvm_class().get_method($name, $sig).unwrap();

    match unsafe { $s.as_jvm_object().call_bool_method(&method, $args) } {
      Ok(b) => Ok(b),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  });
  (int: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = Self::jvm_class().get_method($name, $sig).unwrap();

    match unsafe { $s.as_jvm_object().call_int_method(&method, $args) } {
      Ok(i) => Ok(i),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  });
  (string: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = Self::jvm_class().get_method($name, $sig).unwrap();

    match unsafe { $s.as_jvm_object().call_object_method(&method, $args) } {
      Ok(o) => Ok(o.map({ |h| unsafe { jvm::String::from_object(h) }.to_string() })),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  });
  (nonnull $rty:tt: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = Self::jvm_class().get_method($name, $sig).unwrap();

    match unsafe { $s.as_jvm_object().call_object_method(&method, $args) } {
      Ok(o) => Ok($rty::from_jvm_object(o.unwrap())),
      Err(e) => Err(java::lang::Throwable::from_jvm_object(e))
    }
  })
}

pub mod java;
pub mod jvm;
