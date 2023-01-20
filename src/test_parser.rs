use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

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
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
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
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.instructions(instructions);
        builder.build().unwrap()
    }
}

impl Instruction {
    pub fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        println!("len: {}", pair.clone().into_inner().count());
        for pair in pair.into_inner() {
            // get inner values before assigning them to their concrete type
            let payload = InstructionPayload::parse_from(pair.clone());

            println!("{:?}", pair.as_rule());
            match pair.as_rule() {
                Rule::InstructionLaunch => return Instruction::LaunchProcess(payload),
                Rule::InstructionStdin => return Instruction::SendStdin(payload),
                Rule::InstructionStdout => return Instruction::ExpectStdout(payload),
                Rule::InstructionRegex => return Instruction::ExpectRegex(payload),
                Rule::InstructionControlChar => return Instruction::SendControlChar(payload),
                Rule::InstructionExitCode => return Instruction::ExpectExitCode(payload),
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            };
        }
        unreachable!()
    }
}

impl InstructionPayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::InstructionLaunch
            | Rule::InstructionStdin
            | Rule::InstructionStdout
            | Rule::InstructionRegex => {
                InstructionPayload::StringPayload(StringPayload::parse_from(pair))
            }
            Rule::InstructionControlChar => {
                InstructionPayload::CharacterPayload(CharacterPayload::parse_from(pair))
            }
            Rule::InstructionExitCode => {
                InstructionPayload::ExitCodePayload(ExitCodePayload::parse_from(pair))
            }
            _ => unreachable!(),
        }
    }
}

impl StringPayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = StringPayloadBuilder::default();
        builder.process_id(0); // set default value for syntactic sugar

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcessNumber => {
                    builder.process_id(ProcessID::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::Payload => {
                    builder.string(pair.as_str().into());
                }
                _ => unreachable!(),
            }
        }

        builder.build().unwrap()
    }
}

impl CharacterPayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = CharacterPayloadBuilder::default();
        builder.process_id(0); // set default value for syntactic sugar

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ControlChar => {
                    builder.process_id(ProcessID::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::Payload => {
                    builder.character(pair.as_str().chars().nth(0).unwrap());
                }
                _ => unreachable!(),
            }
        }

        builder.build().unwrap()
    }
}

impl ExitCodePayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = ExitCodePayloadBuilder::default();
        builder.process_id(0); // set default value for syntactic sugar

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcessNumber => {
                    builder.process_id(ProcessID::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::ExitCode => {
                    builder.exit_code(ExitCode::from_str_radix(pair.as_str(), 10).unwrap());
                }
                _ => unreachable!(),
            }
        }

        builder.build().unwrap()
    }
}

pub fn get_pairs(input: &str) -> Pairs<Rule> {
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
