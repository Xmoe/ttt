use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

use crate::ast::*;
use crate::common::*;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to project `src`
pub struct TestFileParser;

type PestError = pest::error::Error<Rule>;

pub fn parse_to_ast(input: &str) -> Result<TestSuite, PestError> {
    let mut pairs = TestFileParser::parse(Rule::TestSuite, input)?;
    let pair = pairs.nth(0).unwrap();
    Ok(TestSuite::parse_from(pair))
}


impl TestSuite {
    pub fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = TestSuiteBuilder::default();
        builder.name("hardcoded lol".into());

        let mut test_cases = vec![];

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::TestCase => {
                    test_cases.push(TestCase::parse_from(pair));
                }
                _ => unreachable!(),
            }
        }

        builder.test_cases(test_cases);
        builder.build().unwrap()
    }
}

impl TestCase {
    pub fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = TestCaseBuilder::default();
        let mut instructions = vec![];

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::TestCaseName => {
                    builder.name(pair.as_str().into());
                }
                Rule::Instruction => instructions.push(Instruction::parse_from(pair)),
                _ => unreachable!(),
            }
        }

        builder.instructions(instructions);
        builder.build().unwrap()
    }
}

impl Instruction {
    pub fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = InstructionBuilder::default();
        builder.process_id(0);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcessNumber => {
                    builder.process_id(u8::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::Payload => {
                    builder.payload(pair.as_str().into());
                }
                Rule::InstructionIdentifier => {
                    builder.kind(InstructionType::parse_from(
                        pair.into_inner().nth(0).unwrap(),
                    ));
                }
                _ => unreachable!(),
            }
        }

        builder.build().unwrap()
    }
}

impl InstructionType {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::IdentifierLaunch => InstructionType::LaunchProcess,
            Rule::IdentifierStdin => InstructionType::PutStdin,
            Rule::IdentifierStdout => InstructionType::ExpectStdout,
            Rule::IdentifierRegex => InstructionType::ExpectRegex,
            Rule::IdentifierControlChar => InstructionType::SendControlCharacter,
            Rule::IdentifierExitCode => InstructionType::ExpectExitCode,
            _ => unreachable!(),
        }
    }
}








pub fn get_pairs(input: &str) -> Pairs<Rule>{
    TestFileParser::parse(Rule::TestSuite, input).unwrap()
}

pub fn print_tree(input: &str) -> Result<(), PestError> {
    let pairs = TestFileParser::parse(Rule::TestSuite, input)?;

    for pair in pairs {
        _print_tree(pair, 0);
    }

    Ok(())
}

fn _print_tree(pair: pest::iterators::Pair<Rule>, depth: u32) {
    for _ in 0..depth {
        print!("  ");
    }

    print!("{:?}", pair.as_rule());
    if pair.clone().into_inner().count() == 0 {
        print!(": {}", pair.as_str());
    }
    println!();

    let pairs = pair.into_inner();
    for pair in pairs {
        _print_tree(pair, depth + 1)
    }
}
