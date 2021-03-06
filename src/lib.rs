// Copyright 2016 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

#![crate_type = "lib"]
#![feature(test)]
#![feature(optin_builtin_traits)]
#![feature(btree_range, collections_bound)]
#![feature(fnbox)]
#![feature(alloc)]
#![feature(plugin)]
#![feature(slice_patterns)]
#![feature(box_syntax)]
#![feature(const_fn)]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(not(feature = "dev"), allow(unknown_lints))]
#![recursion_limit="100"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
extern crate test;
extern crate protobuf;
extern crate bytes;
extern crate byteorder;
extern crate rand;
extern crate mio;
extern crate tempdir;
extern crate rocksdb;
extern crate uuid;
extern crate kvproto;
extern crate time;
extern crate tipb;
extern crate threadpool;
extern crate num;
extern crate cadence;
extern crate libc;
extern crate crc;
extern crate rustc_serialize;
extern crate hyper;
#[cfg(unix)]
extern crate nix;
extern crate alloc;
extern crate chrono;

#[macro_use]
pub mod util;
pub mod raft;
pub mod storage;

pub use storage::{Storage, Dsn};
pub mod raftstore;
pub mod pd;
pub mod server;
