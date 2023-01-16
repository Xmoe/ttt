use thiserror::Error;

pub struct Instruction {
    pub kind: InstructionType,
    pub process_id: u8,
    pub payload: String,
}

pub trait InstructionInteractions {
    fn new(kind: InstructionType, process_id: u8, payload: String) -> Self;
}

impl InstructionInteractions for Instruction {
    fn new(kind: InstructionType, process_id: u8, payload: String) -> Self {
        Instruction {
            kind,
            process_id,
            payload,
        }
    }
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
