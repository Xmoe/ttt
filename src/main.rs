use std::fs::{self, read_to_string};

use anyhow::Result;
use ttt::test_parser::print_tree;

pub mod common;
pub mod test_parser;
pub mod test_runner;

//extern crate pest_derive;

fn main() -> Result<()> {
    let test_file_data = fs::read_to_string("sample.test")?;

    //print_tree(&test_file_data)?;

    let test_suite = test_parser::parse(&test_file_data)?;
    println!("{:#?}", test_suite);

    Ok(())
}
