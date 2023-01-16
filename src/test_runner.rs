use crate::common::*;

use rexpect::session::PtySession;
use std::collections::HashMap;
use std::time::Duration;

pub struct TestRunner {
    timeout: Duration,
    processes: HashMap<u8, PtySession>,
    instructions: Vec<Instruction>,
    variables: HashMap<String, String>,
}

pub trait TestRunnerInteractions {
    fn new(instructions: Vec<Instruction>) -> Self;
    fn run(self) -> Result<(), TestRunnerError>;
    fn set_variables(&self, variables: HashMap<String, String>);
}

impl TestRunnerInteractions for TestRunner {
    fn new(instructions: Vec<Instruction>) -> TestRunner {
        TestRunner {
            timeout: Duration::from_secs(5),
            processes: HashMap::new(),
            instructions,
            variables: HashMap::new(),
        }
    }

    // TODO: How to handle escape characters?

    fn run(mut self) -> Result<(), TestRunnerError> {
        for instruction in self.instructions {
            match instruction {
                Instruction::LaunchProcess(payload, process_id) => {
                    let process = rexpect::spawn(
                        &payload,
                        Some(self.timeout.as_millis() as u64),
                    )?;
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
                Instruction::SendControlCharacter(payload, process_id) => todo!(),
                Instruction::ExpectExitCode(payload, process_id) => todo!(),
                Instruction::SetTimeout(payload) => todo!(),
                Instruction::SetVariable(payload) => todo!(),
            }
        }

        Ok(())
    }

    fn set_variables(&self, variables: HashMap<String, String>) {
        todo!()
    }
}
