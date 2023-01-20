mod common;
mod test_parser;

use anyhow::Result;
use std::fs;
use test_parser::parse_to_ast;

//extern crate pest_derive;

fn main() -> Result<()> {
    let test_file_data = fs::read_to_string("sample.test")?;

    test_parser::print_tree(&test_file_data)?;

    let tree = parse_to_ast(&test_file_data)?;
    println!("{tree:#?}");


    Ok(())
}
