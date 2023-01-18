use pest::Parser;
use pest_derive::Parser;

use crate::common::*;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to project `src`
pub struct TestFileParser;

type PestError = pest::error::Error<Rule>;

pub fn parse(input: &str) -> Result<TestSuite, PestError> {
    let mut pairs = TestFileParser::parse(Rule::TestSuite, input)?;
    let pair = pairs.next().unwrap();

    Ok(parse_test_suite(pair)?)
}

fn parse_test_suite(pair: pest::iterators::Pair<Rule>) -> Result<TestSuite, PestError> {
    let mut test_suite_builder = TestSuiteBuilder::default();
    test_suite_builder.name("hardcoded lol".into());
    let mut test_cases = vec![];

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::TestCase => {
                test_cases.push(parse_test_case(pair));
            }
            _ => unreachable!("test_cases: {:#?}", pair.as_rule()),
        }
    }

    test_suite_builder.test_cases(test_cases);
    Ok(test_suite_builder.build().unwrap())
}

fn parse_test_case(pair: pest::iterators::Pair<Rule>) -> TestCase {
    let mut test_case_builder = TestCaseBuilder::default();
    let mut instructions = vec![];

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::TestCaseName => {
                test_case_builder.name(pair.as_str().to_owned());
            }
            Rule::Instruction => {
                instructions.push(parse_instruction(pair));
            }
            _ => unreachable!(),
        }
    }

    test_case_builder.instructions(instructions);
    test_case_builder.build().unwrap()
}

fn parse_instruction(pair: pest::iterators::Pair<Rule>) -> Instruction {
    let mut process_id = None; // Actually optional as syntactic sugar for process 0
    let mut payload = None;

    let mut rule = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Payload => payload = Some(pair.as_str().to_owned()),
            Rule::ProcessNumber => process_id = Some(pair.as_str().parse::<u8>().unwrap()),
            Rule::InstructionIdentifier => rule = Some(pair.into_inner().next().unwrap().as_rule()),
            _ => unreachable!(),
        }
    }

    if let Some(rule) = rule {
        if let Some(payload) = payload {
            // default process is 0 if no specific ID is given
            let process_id = process_id.unwrap_or(0);

            match rule {
                Rule::IdentifierLaunch => return Instruction::LaunchProcess(payload, process_id),
                Rule::IdentifierStdin => return Instruction::PutStdin(payload, process_id),
                Rule::IdentifierStdout => return Instruction::ExpectStdout(payload, process_id),
                Rule::IdentifierRegex => return Instruction::ExpectRegex(payload, process_id),
                Rule::IdentifierControlChar => {
                    // TODO: should encode valid control characters in grammar
                    if payload.len() != 1 {
                        panic!("Control char must be exactly length one")
                    }
                    let control_char = payload.chars().next().unwrap();

                    return Instruction::SendControlCharacter(control_char, process_id);
                }
                Rule::IdentifierExitCode => {
                    let exit_code = payload.parse::<i32>().unwrap();

                    return Instruction::ExpectExitCode(exit_code, process_id);
                }
                _ => unreachable!(),
            }
        }
    }
    unreachable!(
        "Reached unreachable: rule: {:?}, payload: {:?}, id: {:?}",
        rule, payload, process_id
    );
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
