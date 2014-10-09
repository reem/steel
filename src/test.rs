use std::time::duration::Duration;
use std::task;
use run::Run;

use {time};

pub struct Test {
    pub name: String,
    test: fn()
}

pub enum TestReport {
    Passed(Duration),
    Failed
}

impl Run for Test {
    type Report = TestReport;

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

