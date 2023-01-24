use ttt::common::*;
use ttt::test_parser;
use ttt::test_parser::*;
use ttt::test_runner::*;

#[test]
fn sample_test() {
    let test_data = "[Test 1]\n\
                            $ echo blub\n\
                            > blub\n\
                            ? 0\n";

    let test_suite = test_parser::parse(test_data).unwrap();

    let runner = TestSuiteRunner::new(test_suite);
    runner.run();
}

#[test]
fn use_each_instruction() {
    let test_data = "[Test 2]\n\
    0$ echo blub\n\
    0> blub\n\
    1$ cat\n\
    1< foo bar\n\
    1r foo bar\n\
    1^ d\n\
    1? 0\n";

    let test_suite = test_parser::parse(test_data).unwrap();

    let runner = TestSuiteRunner::new(test_suite);
    runner.run();
}
