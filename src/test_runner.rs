use crate::common::*;

use rexpect::process::wait::WaitStatus;
use rexpect::session::PtySession;
use std::collections::HashMap;
use std::time::Duration;

pub struct TestRunner {
    timeout: Duration,
    processes: HashMap<u8, PtySession>,
    test_case: SingleTest,
    variables: HashMap<String, String>,
}

pub trait TestRunnerInteractions {
    fn new(instructions: SingleTest) -> Self;
    fn run(self) -> Result<(), TestRunnerError>;
    fn set_variables(&self, variables: HashMap<String, String>);
}

impl TestRunnerInteractions for TestRunner {
    fn new(test_case: SingleTest) -> TestRunner {
        TestRunner {
            timeout: Duration::from_secs(5),
            processes: HashMap::new(),
            test_case,
            variables: HashMap::new(),
        }
    }

    fn run(mut self) -> Result<(), TestRunnerError> {
        for instruction in self.test_case.instructions {
            match instruction {
                Instruction::LaunchProcess(payload, process_id) => {
                    let process = rexpect::spawn(&payload, Some(self.timeout.as_millis() as u64))?;
                    self.processes.insert(process_id, process);
                }

                Instruction::ExpectStdout(payload, process_id) => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    let _ = process.exp_string(&payload)?;
                    println!("Successfully found '{}'", payload);
                }

                Instruction::PutStdin(payload, process_id) => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_line(&payload)?;
                }

                Instruction::ExpectRegex(payload, process_id) => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.exp_regex(&payload)?;
                }

                Instruction::SendControlCharacter(char, process_id) => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_control(char)?;
                }
                
                Instruction::ExpectExitCode(expected_exit_code, process_id) => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;


                    //TODO: maybe set default timeout again for safety, because wait is blocking!
                    if let Ok(WaitStatus::Exited(_, exit_code)) = process.process.wait() {
                        if exit_code == expected_exit_code {
                            return Ok(())
                        }
                    }

                    return Err(TestRunnerError::WrongExitCode)
                }
                //Instruction::SetTimeout(payload) => todo!(),
                //Instruction::SetVariable(payload) => todo!(),
            }
        }

        Ok(())
    }

    fn set_variables(&self, variables: HashMap<String, String>) {
        todo!()
    }
}
