mod common;
mod test_runner;

use crate::common::*;
use crate::test_runner::*;

fn main() {
    let test_instructions = vec![
        Instruction {
            kind: InstructionType::LaunchProcess,
            process_id: 0,
            payload: "uname".into(),
        },
        Instruction {
            kind: InstructionType::ExpectStdout,
            process_id: 0,
            payload: "Linux".into(),
        },
    ];

    let runner = TestRunner::new(test_instructions);
    let res = runner.run();
}
