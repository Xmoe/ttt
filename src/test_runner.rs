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
        for instruction in &self.instructions {
            match instruction.kind {
                InstructionType::LaunchProcess => {
                    let process = rexpect::spawn(
                        &instruction.payload,
                        Some(self.timeout.as_millis() as u64),
                    )?;
                    self.processes.insert(instruction.process_id, process);
                }

                InstructionType::ExpectStdout => {
                    let process = self
                        .processes
                        .get_mut(&instruction.process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    let _ = process.exp_string(&instruction.payload)?;
                    println!("Successfully found '{}'", instruction.payload);
                }

                InstructionType::PutStdin => {
                    let process = self
                        .processes
                        .get_mut(&instruction.process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_line(&instruction.payload)?;
                }

                InstructionType::ExpectRegex => todo!(),
                InstructionType::SendControlCharacter => todo!(),
                InstructionType::ExpectExitCode => todo!(),
                InstructionType::SetTimeout => todo!(),
                InstructionType::SetVariable => todo!(),
            }
        }

        Ok(())
    }

    fn set_variables(&self, variables: HashMap<String, String>) {
        todo!()
    }
}
