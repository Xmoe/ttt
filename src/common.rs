use derive_builder::Builder;

pub type ProcessID = u8;
pub type ExitCode = i32;

/// A collection of all test cases in a file
#[derive(Builder, Debug)]
pub struct TestSuite {
    pub name: String,
    pub test_cases: Vec<TestCase>,
}

/// A single test case
#[derive(Builder, Clone, Debug)]
pub struct TestCase {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    LaunchProcess {
        string: String,
        process_id: ProcessID,
    },
    SendStdin {
        string: String,
        process_id: ProcessID,
    },
    ExpectStdout {
        string: String,
        process_id: ProcessID,
    },
    ExpectRegex {
        string: String,
        process_id: ProcessID,
    },
    SendControlChar {
        character: char,
        process_id: ProcessID,
    },
    ExpectExitCode {
        modifier: ExitCodeModifier,
        exit_code: ExitCode,
        process_id: ProcessID,
    },
}

#[derive(Debug, Clone)]
pub enum ExitCodeModifier {
    Equals,
    LessThan,
    MoreThan,
}
