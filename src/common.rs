use derive_builder::Builder;

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

pub type ProcessID = u8;
pub type ExitCode = i32;

#[derive(Debug, Clone)]
pub enum Instruction {
    LaunchProcess(InstructionPayload),
    SendStdin(InstructionPayload),
    ExpectStdout(InstructionPayload),
    ExpectRegex(InstructionPayload),
    SendControlChar(InstructionPayload),
    ExpectExitCode(InstructionPayload),
}

// Dear god of abstraction
// https://stackoverflow.com/questions/72438594/how-can-i-use-enum-variants-as-generic-type
#[derive(Debug, Clone)]
pub enum InstructionPayload {
    StringPayload(StringPayload),
    CharacterPayload(CharacterPayload),
    ExitCodePayload(ExitCodePayload),
}

#[derive(Debug, Clone, Builder)]
pub struct StringPayload {
    pub string: String,
    pub process_id: ProcessID,
}

#[derive(Debug, Clone, Builder)]
pub struct     CharacterPayload {
    pub character: char,
    pub process_id: ProcessID,
}

#[derive(Debug, Clone, Builder)]
pub struct     ExitCodePayload {
    pub exit_code: ExitCode,
    pub process_id: ProcessID,
}