#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]
#![feature(associated_types)]

//! Crate comment goes here

extern crate "test" as testcrate;
extern crate time;

mod run;
mod suite;
mod test;

