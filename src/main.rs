mod common;
mod test_runner;

use crate::common::*;
use crate::test_runner::*;

fn main() {
    let test_instructions = vec![
        Instruction::new(InstructionType::LaunchProcess, 0, "uname".into()),
        Instruction::new(InstructionType::PutStdin, 0, "wololo".into()),
        Instruction::new(InstructionType::ExpectStdout, 0, "Linux".into()),
    ];

    let runner = TestRunner::new(test_instructions);
    let res = runner.run();
}
