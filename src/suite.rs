use std::{iter, slice};

use test::{Test, TestReport};
use run::Run;

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

    pub fn run<'a>(&'a self) -> SuiteReports<'a> {
        (&self).run()
    }
}

impl<'a> Run for &'a Suite {
    // Hack because no HKL...
    type Report = SuiteReports<'a>;

    fn run(&self) -> SuiteReports<'a> {
        SuiteReports {
            stack: vec![*self],
            current: None,
            first: true
        }
    }
}

pub enum SuiteReport<'a> {
    SuiteStart(&'a Suite),
    SuiteEnd,
    TestRun(&'a Test, TestReport),
}

pub struct SuiteReports<'a> {
    first: bool,
    stack: Vec<&'a Suite>,
    current: Option<SuiteIterator<'a>>
}

impl<'a> Iterator<SuiteReport<'a>> for SuiteReports<'a> {
    fn next(&mut self) -> Option<SuiteReport<'a>> {
        match self.current {
            Some(ref mut current) => {
                match current.next() {
                    None => {}
                    Some((test, report)) => return Some(TestRun(test, report))
                }
            },

            None => {}
        };

        // No more tests in the current suite or no current.
        let next = match self.stack.pop() {
            Some(next) => next,
            None => return None
        };

        self.stack.extend(next.subsuites.iter());
        self.current = Some(SuiteIterator::new(&*next));

        if self.first {
            self.first = false;
            Some(SuiteStart(next))
        } else {
            Some(SuiteEnd)
        }
    }
}

struct SuiteIterator<'a> {
    tests: iter::Map<'static, &'a Test, (&'a Test, TestReport), slice::Items<'a, Test>>
}

impl<'a> SuiteIterator<'a> {
    fn new(suite: &'a Suite) -> SuiteIterator<'a> {
        fn run(test: &Test) -> (&Test, TestReport) {
           (test, test.run())
        }

        SuiteIterator {
            tests: suite.tests.iter().map(run)
        }
    }
}

impl<'a> Iterator<(&'a Test, TestReport)> for SuiteIterator<'a> {
    fn next(&mut self) -> Option<(&'a Test, TestReport)> {
        self.tests.next()
    }
}

