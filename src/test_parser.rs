use std::collections::HashMap;

use derive_builder::Builder;
use pest::Parser;
use pest_derive::Parser;

use crate::common::*;

#[derive(Parser)]
#[grammar = "grammar.pest"] // relative to project `src`
pub struct TestFileParser;

type PestError = pest::error::Error<Rule>;

trait ParseTreeToType {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self;
}

pub fn parse(input: &str) -> Result<TestSuite, PestError> {
    let mut pairs = TestFileParser::parse(Rule::TestSuite, input)?;
    let pair = pairs.next().unwrap();
    Ok(TestSuite::parse_from(pair))
}

impl ParseTreeToType for TestSuite {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = TestSuiteBuilder::default();
        // TODO change hardcoded stuff
        builder.name("hardcoded lol".into());
        let mut variables: HashMap<String, String> = HashMap::new();

        let mut test_cases = vec![];

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::TestCase => {
                    test_cases.push(TestCase::parse_from(pair));
                }
                Rule::VariableInitialization => {
                    let var = VarParserHelper::parse_from(pair);
                    variables.insert(var.variable, var.value);
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.variables(variables);
        builder.test_cases(test_cases);
        builder.build().unwrap()
    }
}

impl ParseTreeToType for TestCase {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
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

impl ParseTreeToType for Instruction {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::InstructionLaunch => {
                    let payload = LaunchPayload::parse_from(pair);
                    return Instruction::LaunchProcess {
                        string: payload.string,
                        process_id: payload.process_id,
                        variable: payload.variable,
                    };
                }
                Rule::InstructionStdin => {
                    let payload = StringPayload::parse_from(pair);
                    return Instruction::SendStdin {
                        string: payload.string,
                        process_id: payload.process_id,
                    };
                }
                Rule::InstructionStdout => {
                    let payload = StringPayload::parse_from(pair);

                    return Instruction::ExpectStdout {
                        string: payload.string,
                        process_id: payload.process_id,
                    };
                }
                Rule::InstructionRegex => {
                    let payload = StringPayload::parse_from(pair);

                    return Instruction::ExpectRegex {
                        string: payload.string,
                        process_id: payload.process_id,
                    };
                }
                Rule::InstructionControlChar => {
                    let payload = CharacterPayload::parse_from(pair);

                    return Instruction::SendControlChar {
                        character: payload.character,
                        process_id: payload.process_id,
                    };
                }
                Rule::InstructionExitCode => {
                    let payload = ExitCodePayload::parse_from(pair);

                    return Instruction::ExpectExitCode {
                        modifier: payload.modifier,
                        exit_code: payload.exit_code,
                        process_id: payload.process_id,
                    };
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            };
        }
        unreachable!()
    }
}

// These are helper types to make parsing nicer

#[derive(Debug, Clone, Builder)]
struct VarParserHelper {
    pub variable: String,
    pub value: String,
}

#[derive(Debug, Clone, Builder)]
struct StringPayload {
    pub string: String,
    pub process_id: ProcessID,
}

#[derive(Debug, Clone, Builder)]
struct CharacterPayload {
    pub character: char,
    pub process_id: ProcessID,
}

#[derive(Debug, Clone, Builder)]
struct ExitCodePayload {
    pub modifier: ExitCodeModifier,
    pub exit_code: ExitCode,
    pub process_id: ProcessID,
}

#[derive(Debug, Clone, Builder)]
struct LaunchPayload {
    pub variable: Option<String>,
    pub string: String,
    pub process_id: ProcessID,
}

impl ParseTreeToType for StringPayload {
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
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.build().unwrap()
    }
}

impl ParseTreeToType for CharacterPayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = CharacterPayloadBuilder::default();
        builder.process_id(0); // set default value for syntactic sugar

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcessNumber => {
                    builder.process_id(ProcessID::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::ControlChar => {
                    builder.character(pair.as_str().chars().next().unwrap());
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.build().unwrap()
    }
}

impl ParseTreeToType for ExitCodePayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = ExitCodePayloadBuilder::default();
        // set default values for syntactic sugar
        builder.process_id(0);
        builder.modifier(ExitCodeModifier::Equals);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcessNumber => {
                    builder.process_id(ProcessID::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::ExitCode => {
                    builder.exit_code(ExitCode::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::ExitCodeModifier => {
                    builder.modifier(match pair.as_str() {
                        "<" => ExitCodeModifier::LessThan,
                        ">" => ExitCodeModifier::MoreThan,
                        _ => ExitCodeModifier::Equals,
                    });
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.build().unwrap()
    }
}

impl ParseTreeToType for LaunchPayload {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = LaunchPayloadBuilder::default();
        // set default values for syntactic sugar
        builder.process_id(0);
        builder.variable(None);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::ProcessNumber => {
                    builder.process_id(ProcessID::from_str_radix(pair.as_str(), 10).unwrap());
                }
                Rule::Payload => {
                    builder.string(pair.as_str().to_string());
                }
                Rule::Variable => {
                    builder.variable(Some(pair.as_str().to_string()));
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.build().unwrap()
    }
}

impl ParseTreeToType for VarParserHelper {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut builder = VarParserHelperBuilder::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::Variable => {
                    builder.variable(pair.as_str().to_string());
                }
                Rule::Value => {
                    builder.value(pair.as_str().to_string());
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }

        builder.build().unwrap()
    }
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
