use thiserror::Error;

pub struct TestGroup {
    test_file_name: String,
    tests: Vec<SingleTest>,
}

pub struct SingleTest {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

type ProcessID = u8;
type Payload = String;

pub enum Instruction {
    LaunchProcess(Payload, ProcessID),     // $
    PutStdin(Payload, ProcessID),          // <
    ExpectStdout(Payload, ProcessID),      // >
    ExpectRegex(Payload, ProcessID),       // ~
    SendControlCharacter(char, ProcessID), // ^
    ExpectExitCode(i32, ProcessID),         // ?
                                           //SetTimeout(Payload),                      // t
                                           //SetVariable(Payload),                     // =
}

#[derive(Error, Debug)]
pub enum TestRunnerError {
    #[error("Invalid Process ID")]
    InvalidProcess,
    #[error("Wrong exit code")]
    WrongExitCode,
    #[error(transparent)]
    RexpectError(#[from] rexpect::error::Error),
}
