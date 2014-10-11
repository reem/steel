use std::time::duration::Duration;
use std::task;
use run::Run;

use {time};

#[deriving(Clone)]
pub struct Test {
    pub name: String,
    pub test: fn()
}

impl Test {
    pub fn new(name: String, test: fn()) -> Test {
        Test { name: name, test: test }
    }
}

pub enum TestReport {
    Passed(Duration),
    Failed
}

impl Run<TestReport> for Test {
    fn run(&self) -> TestReport {
        let test = self.test;
        let result = task::try(proc() {
            let start = time::precise_time_ns();
            test();
            Duration::nanoseconds((time::precise_time_ns() - start) as i64)
        });

        match result {
            Ok(dur) => Passed(dur),
            _ => Failed
        }
    }
}

