use thiserror::Error;

type ProcessID = u8;
type Payload = String;

pub enum Instruction {
    LaunchProcess(Payload, ProcessID),        // $
    PutStdin(Payload, ProcessID),             // <
    ExpectStdout(Payload, ProcessID),         // >
    ExpectRegex(Payload, ProcessID),          // ~
    SendControlCharacter(Payload, ProcessID), // ^
    ExpectExitCode(Payload, ProcessID),       // ?
    SetTimeout(Payload),                      // t
    SetVariable(Payload),                     // =
}

#[derive(Error, Debug)]
pub enum TestRunnerError {
    #[error("Invalid Process ID")]
    InvalidProcess,
    #[error(transparent)]
    RexpectError(#[from] rexpect::error::Error),
}
