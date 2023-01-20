mod common;
mod test_parser;
mod test_runner;
mod ast;

use std::{env::current_dir, fs};

use anyhow::Result;
use ttt::test_parser::get_pairs;

use crate::test_parser::parse_to_ast;


//extern crate pest_derive;

fn main() -> Result<()> {
    let test_file_data = fs::read_to_string("sample.test")?;
    
    test_parser::print_tree(&test_file_data)?;

    //let x = get_pairs(&test_file_data);
    //println!("{:#?}", x);

    let tree =  parse_to_ast(&test_file_data)?;
    println!("{tree:#?}");
    /*
    match tree {
        ast::TestSuiteNode::TestCases(test_cases) => {
            for case in test_cases {
                println!("{:#?}", case);
            }
        },
    }

    */
    //let test_suite: common::TestSuite = test_parser::parse(&test_file_data)?;
    //let runner = test_runner::TestSuiteRunner::new(test_suite);
    //runner.run();

    //println!("{:#?}", test_suite);


    Ok(())
}
