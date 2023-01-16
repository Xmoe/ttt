mod common;
mod test_runner;

use crate::common::*;
use crate::test_runner::*;

fn main() {
    let test_instructions = vec![
        Instruction::LaunchProcess("uname".into(), 0),
        Instruction::PutStdin("wololo".into(), 0),
        Instruction::ExpectStdout("Linux".into(), 0),
    ];

    let runner = TestRunner::new(test_instructions);
    let res = runner.run();
}
