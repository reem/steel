use std::vec::MoveItems;

use test::{Test, TestReport};
use run::Run;

use self::SuiteReportsState::*;
use self::SuiteReport::*;

#[deriving(Clone)]
pub struct Suite {
    pub name: String,
    pub tests: Vec<Test>,
    pub subsuites: Vec<Suite>
}

impl Suite {
    pub fn new(name: String) -> Suite {
        Suite {
            name: name,
            tests: vec![],
            subsuites: vec![]
        }
    }
}

impl Run<SuiteReports> for Suite {
    fn run(&self) -> SuiteReports {
        SuiteReports {
            state: Starting,
            name: self.name.clone(),
            tests: self.tests.clone().into_iter(),
            suites: self.subsuites.clone().into_iter(),
            subsuite: None
        }
    }
}

pub enum SuiteReport {
    SuiteStart(String),
    SuiteEnd(String),
    TestRun(Test, TestReport),
}

pub struct SuiteReports {
    state: SuiteReportsState,
    name: String,
    tests: MoveItems<Test>,
    suites: MoveItems<Suite>,
    subsuite: Option<Box<SuiteReports>>
}

enum SuiteReportsState {
    Starting,
    Testing,
    SubRunning,
    Ending,
    Ended
}

impl Iterator<SuiteReport> for SuiteReports {
    fn next(&mut self) -> Option<SuiteReport> {
        match self.state {
            Starting => {
                self.state = Testing;
                Some(SuiteStart(self.name.clone()))
            },

            Testing => {
                match self.tests.next() {
                    Some(test) => Some(TestRun(test.clone(), test.run())),
                    None => {
                        self.state = SubRunning;
                        self.next()
                    }
                }
            },

            SubRunning => {
                match self.subsuite {
                    Some(box ref mut iter) => {
                        match iter.next() {
                            Some(report) => return Some(report),
                            None => {}
                        };

                        match self.suites.next() {
                            Some(next) => *iter = next.run(),
                            None => self.state = Ending,
                        };
                    },
                    None => {
                        match self.suites.next() {
                            Some(suite) => self.subsuite = Some(box suite.run()),
                            None => self.state = Ending,
                        };
                    }
                };

                self.next()
            },

            Ending => {
                self.state = Ended;
                Some(SuiteEnd(self.name.clone()))
            },

            Ended => None
        }
    }
}

