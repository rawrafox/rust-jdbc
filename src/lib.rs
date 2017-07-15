// Copyright 2017 The rust-jdbc developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate jni_sys;

#[link(name="jvm")]
extern { }

macro_rules! java_call {
  (static, nonnull $rty:tt: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = $s.class.get_static_method($name, $sig).unwrap();

    match unsafe { $s.class.call_object_method(&method, $args) } {
      Ok(o) => Ok($rty::new(&$s.environment, o.unwrap())),
      Err(e) => Err(e)
    }
  });
  (bool: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = $s.class.get_method($name, $sig).unwrap();

    unsafe { $s.object.call_bool_method(&method, $args) }
  });
  (int: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = $s.class.get_method($name, $sig).unwrap();

    unsafe { $s.object.call_int_method(&method, $args) }
  });
  (string: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = $s.class.get_method($name, $sig).unwrap();

    match unsafe { $s.object.call_object_method(&method, $args) } {
      Ok(o) => Ok(o.map({ |h| unsafe { String::from_object(h) }.to_string() })),
      Err(e) => Err(e)
    }
  });
  (nonnull $rty:tt: $s:expr, $name:expr, $sig:expr, $args:expr) => ({
    let method = $s.class.get_method($name, $sig).unwrap();

    match unsafe { $s.object.call_object_method(&method, $args) } {
      Ok(o) => Ok($rty::new(&$s.environment, o.unwrap())),
      Err(e) => Err(e)
    }
  })
}

mod connection;
mod context;
mod driver_manager;
mod jvm;
mod result_set;
mod result_set_meta_data;
mod statement;

pub use connection::*;
pub use context::*;
pub use driver_manager::*;
pub use result_set::*;
pub use result_set_meta_data::*;
pub use statement::*;
