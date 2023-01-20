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

#[derive(Builder, Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionType,
    pub process_id: u8,
    pub payload: String,
}

#[derive(Clone, Debug)]
pub enum InstructionType {
    LaunchProcess,        // $
    PutStdin,             // <
    ExpectStdout,         // >
    ExpectRegex,          // r
    SendControlCharacter, // ^
    ExpectExitCode,       // ?
                          //SetTimeout(Payload),                   // t
                          //SetVariable(Payload),                     // =
}
