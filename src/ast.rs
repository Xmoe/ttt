use std::{iter::Inspect, num::ParseIntError, vec};

// Rule created by pest in mod test_parser
use crate::{
    common::*,
    test_parser::Rule,
};

#[derive(Debug)]
pub enum TestSuiteNode {
    // Here could be _your_ top level token!
    TestCases(Vec<TestCaseNode>),
}

#[derive(Debug)]
pub enum TestCaseNode {
    Name(String),
    Instructions(Vec<InstructionNode>),
}

#[derive(Debug)]
pub enum InstructionNode {
    ProcessNumber(u8),
    InstructionIdentifier(String),
    Payload(String),
}

pub trait NodeToAst {
    fn parse_from(pairs: pest::iterators::Pairs<Rule>) -> Self;
}

impl TestSuiteNode {
    pub fn parse_from(pairs: pest::iterators::Pairs<Rule>) -> TestSuiteNode {
        let mut test_case_nodes = vec![];
        for pair in pairs {
            println!("{:#?}", pair.as_rule());
            match pair.as_rule() {
                Rule::TestCase => {
                    test_case_nodes.push(TestCaseNode::parse_from(pair));
                }
                _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
            }
        }
        TestSuiteNode::TestCases(test_case_nodes)
    }
}

impl TestCaseNode {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> TestCaseNode {
        // Go one level lower
        let pair = pair.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::TestCaseName => TestCaseNode::Name(pair.as_str().to_owned()),
            Rule::Instruction => {
                let mut instruction_nodes = vec![];
                for pair in pair.into_inner() {
                    println!("{:#?}", pair.as_rule());

                    instruction_nodes.push(InstructionNode::parse_from(pair))
                }
                TestCaseNode::Instructions(instruction_nodes)
            }
            _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
        }
    }
}

impl InstructionNode {
    fn parse_from(pair: pest::iterators::Pair<Rule>) -> InstructionNode {
        let pair = pair.into_inner().nth(0).unwrap();

        match pair.as_rule() {
            Rule::ProcessNumber => {
                InstructionNode::ProcessNumber(u8::from_str_radix(pair.as_str(), 10).unwrap())
            }
            Rule::InstructionIdentifier => {
                InstructionNode::InstructionIdentifier(pair.as_str().to_owned())
            }
            Rule::Payload => InstructionNode::Payload(pair.as_str().to_owned()),
            _ => unreachable!("Rule: {:?} | Content: {}", pair.as_rule(), pair.as_str()),
        }
    }
}
