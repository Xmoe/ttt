use ttt::prelude::*;

#[test]
fn sample_test() -> Result<(), TestRunnerError> {
    let instructions = vec![
        Instruction::LaunchProcess("uname".into(), 0),
        Instruction::ExpectStdout("Linux".into(), 0),
        Instruction::ExpectExitCode(0, 0),
    ];

    let test_case = TestCase {
        name: "Example".into(),
        instructions,
    };

    let runner = TestRunner::new(test_case);
    runner.run()?;

    Ok(())
}

#[test]
fn use_each_instruction() -> Result<(), TestRunnerError> {
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

    let test_case = TestCase {
        name: "Beispiel".into(),
        instructions,
    };

    let runner = TestRunner::new(test_case);
    runner.run()?;

    Ok(())
}

#[test]
#[should_panic]
fn verify_control_character() {
    let instructions = vec![
        Instruction::LaunchProcess("sleep 100".into(), 0),
        Instruction::SendControlCharacter('C', 0),
        Instruction::ExpectExitCode(0, 0),
    ];

    let test_case = TestCase {
        name: "Beispiel".into(),
        instructions,
    };

    let runner = TestRunner::new(test_case);
    runner.run().unwrap();
}
