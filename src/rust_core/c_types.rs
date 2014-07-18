// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Supported:
// - x86-linux-gnu
// - x86_64-linux-gnu
// - arm-linux-gnueabi

#![allow(non_camel_case_types)]

pub type c_short = i16;
pub type c_ushort = u16;

pub type c_int = i32;
pub type c_uint = u32;

#[cfg(target_word_size = "32")]
pub type c_long = i32;
#[cfg(target_word_size = "32")]
pub type c_ulong = u32;

#[cfg(target_word_size = "64")]
pub type c_long = i64;
#[cfg(target_word_size = "64")]
pub type c_ulong = u64;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type clockid_t = i32;

pub type time_t = c_long;

pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: c_long
}

pub struct pthread_t {
    size: c_ulong
}

#[cfg(target_word_size = "32")]
pub struct pthread_attr_t {
    size: [u32, ..9]
}
#[cfg(target_word_size = "64")]
pub struct pthread_attr_t {
    size: [u64, ..7]
}

#[cfg(target_word_size = "32")]
pub struct pthread_mutex_t {
    size: [u32, ..6]
}
#[cfg(target_word_size = "64")]
pub struct pthread_mutex_t {
    size: [u64, ..5]
}

pub struct pthread_mutexattr_t {
    size: u32
}

pub struct pthread_cond_t {
    size: [u64, ..6]
}

pub struct pthread_condattr_t {
    size: u32
}
