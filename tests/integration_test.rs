use ttt::prelude::*;

#[test]
fn sample_test() -> Result<(), TestRunnerError> {
    let instructions = vec![
        Instruction::LaunchProcess("uname".into(), 0),
        Instruction::ExpectStdout("Linux".into(), 0),
        Instruction::ExpectExitCode(0, 0),
    ];

    let test_case = SingleTest {
        name: "Example".into(),
        instructions,
    };

    let runner = TestRunner::new(test_case);
    runner.run()?;

    Ok(())
}
