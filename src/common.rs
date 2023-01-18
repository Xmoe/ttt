use derive_builder::Builder;
use thiserror::Error;

/// A collection of all test cases in a file
#[derive(Builder, Debug)]
pub struct TestSuite {
    name: String,
    test_cases: Vec<TestCase>,
}

/// A single test case
#[derive(Builder, Clone, Debug)]
pub struct TestCase {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

type ProcessID = u8;
type Payload = String;
type ExitCode = i32;

#[derive(Clone, Debug)]
pub enum Instruction {
    LaunchProcess(Payload, ProcessID),     // $
    PutStdin(Payload, ProcessID),          // <
    ExpectStdout(Payload, ProcessID),      // >
    ExpectRegex(Payload, ProcessID),       // r
    SendControlCharacter(char, ProcessID), // ^
    ExpectExitCode(ExitCode, ProcessID),   // ?
                                           //SetTimeout(Payload),                   // t
                                           //SetVariable(Payload),                     // =
}

#[derive(Error, Debug)]
pub enum TestRunnerError {
    #[error("Invalid Process ID")]
    InvalidProcess,
    #[error("Wrong exit code")]
    WrongExitCode,
    #[error("Process killed by signal or somethin")]
    WronglyExited,
    #[error(transparent)]
    RexpectError(#[from] rexpect::error::Error),
}
