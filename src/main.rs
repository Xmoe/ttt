mod common;
mod test_runner;

use crate::common::*;
use crate::test_runner::*;

fn main() {
    let instructions = vec![
        Instruction::LaunchProcess("uname".into(), 0),
        Instruction::PutStdin("wololo".into(), 0),
        Instruction::ExpectStdout("Linux".into(), 0),
        Instruction::ExpectExitCode(0, 0),
        Instruction::LaunchProcess("uname".into(), 1),
        Instruction::ExpectRegex(r"(l|L)inux".into(), 1),
        Instruction::SendControlCharacter('C', 1),
        Instruction::ExpectExitCode(0, 1),
    ];

    let test_case = SingleTest {
        name: "Beispiel".into(),
        instructions,
    };

    let runner = TestRunner::new(test_case);

    if let Ok(_) = runner.run() {
        println!("Test ran successfully!")
    } else {
        println!("Test failed!")
    }
}
