mod common;
mod test_parser;
mod test_runner;

use anyhow::Result;
use std::{fs, collections::HashMap};
use test_parser::{parse, print_tree};
use test_runner::TestSuiteRunner;

//extern crate pest_derive;

fn main() -> Result<()> {
    let test_file_data = fs::read_to_string("sample.test")?;

    let test_suite = parse(&test_file_data)?;

    let runner = TestSuiteRunner::new(test_suite);
    runner.run();

    Ok(())
}
