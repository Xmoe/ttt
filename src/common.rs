use thiserror::Error;

pub struct Instruction {
    pub kind: InstructionType,
    pub process_id: u8,
    pub payload: String,
}

pub enum InstructionType {
    LaunchProcess,        // $
    PutStdin,             // <
    ExpectStdout,         // >
    ExpectRegex,          // ~
    SendControlCharacter, // ^
    ExpectExitCode,       // ?
    SetTimeout,           // t
    SetVariable,          // =
}

#[derive(Error, Debug)]
pub enum TestRunnerError {
    #[error("Invalid Process ID")]
    InvalidProcess,
    #[error(transparent)]
    RexpectError(#[from] rexpect::error::Error),
}