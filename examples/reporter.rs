extern crate steel;

use steel::{Suite, Test};
use steel::suite::{SuiteStart, SuiteEnd, TestRun};
use steel::test::{Passed, Failed};
use steel::run::Run;

fn test() {
    assert!(true)
}

fn main() {
    let testsuite = Suite {
        name: "top".to_string(),

        tests: vec![
            Test {
                name: "first".to_string(),
                test: test
            },

            Test {
                name: "second".to_string(),
                test: test
            }
        ],

        subsuites: vec![
            Suite {
                name: "nested".to_string(),
                tests: vec![Test {
                    name: "nested test".to_string(),
                    test: test
                }],
                subsuites: vec![]
            }
        ]
    };

    let mut indents = 0u;

    for event in testsuite.run() {
        match event {
            SuiteStart(name) => {
                println!("{}Starting Suite {}", whitespace(indents), name.as_slice());
                indents += 1;
            },

            SuiteEnd(name) => {
                indents -= 1;
                println!("{}Ending Suite {}", whitespace(indents), name.as_slice());
            },

            TestRun(test, report) => {
                println!("{}Test {} - {}", whitespace(indents), test.name.as_slice(), match report {
                    Passed(time) => format!("passed in {} ns", time),
                    Failed => "failed".to_string()
                }.as_slice())
            }
        }
    }

    fn whitespace(num: uint) -> String {
        let mut string = "".to_string();
        for _ in range(0, num) {
            string.push_str("  ");
        }
        string
    }
}

