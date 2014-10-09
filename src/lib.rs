#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(warnings)]

//! Crate comment goes here

extern crate "test" as testcrate;
extern crate time;

pub use run::Run;
pub use suite::Suite;
pub use test::Test;

pub mod run;
pub mod suite;
pub mod test;

