use crate::common::*;

use rexpect::process::wait::WaitStatus;
use rexpect::session::{PtySession};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

pub struct TestSuiteRunner {
    test_suite: TestSuite,
}

impl TestSuiteRunner {
    pub fn new(test_suite: TestSuite) -> Self {
        TestSuiteRunner { test_suite }
    }

    // TODO: Add context like programs vars etc to parameters
    pub fn run(self) {
        let mut successes: u32 = 0;
        for (index, test_case) in self.test_suite.test_cases.into_iter().enumerate() {
            let test_runner = TestRunner::new(test_case.clone());
            let text = format!("Running [{}]:", test_case.name);
            print!("{text:<40}");
            match test_runner.run() {
                Ok(()) => {
                    println!("Success!");
                    successes += 1;
                }
                Err(_) => println!("FAIL"),
            }
        }
    }
}

pub struct TestRunner {
    timeout: Duration,
    processes: HashMap<u8, PtySession>,
    test_case: TestCase,
}

impl TestRunner {
    pub fn new(test_case: TestCase) -> Self {
        TestRunner {
            timeout: Duration::from_secs(5),
            processes: HashMap::new(),
            test_case,
        }
    }

    pub fn run(mut self) -> Result<(), TestRunnerError> {
        for instruction in self.test_case.instructions {
            match instruction {
                Instruction::LaunchProcess{string, process_id} => {

                    // rexpect gives no way to check whether a process has been successfully created until something is expected :(
                    // Use rust-psutil to detect aliveness of the program?

                    let process = rexpect::session::spawn(&string, Some(self.timeout.as_millis() as u64))?;
                    if let Some(WaitStatus::StillAlive) = process.process.status() {
                        self.processes.insert(process_id, process);
                    } else {
                        return Err(TestRunnerError::InvalidProcess)
                    };
                }

                Instruction::ExpectStdout{string, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    let _ = process.exp_string(&string)?;
                }

                Instruction::SendStdin{string, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_line(&string)?;
                }

                Instruction::ExpectRegex{string, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.exp_regex(&string)?;
                }

                Instruction::SendControlChar{character, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_control(character)?;
                }

                Instruction::ExpectExitCode{exit_code, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;

                    let expected_exit_code = exit_code;
                    
                    //TODO: maybe set default timeout again for safety, because wait is blocking!
                    if let Ok(WaitStatus::Exited(_, exit_code)) = process.process.wait() {
                        if exit_code != expected_exit_code {
                            return Err(TestRunnerError::WrongExitCode)
                        }
                    } else {
                        return Err(TestRunnerError::WronglyExited)
                    }
                }
                //Instruction::SetTimeout(payload) => todo!(),
                //Instruction::SetVariable(payload) => todo!(),
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum TestRunnerError {
    #[error("Invalid Process ID")]
    InvalidProcess,
    #[error("Wrong exit code")]
    WrongExitCode,
    #[error("Process killed by signal or somethin")]
    WronglyExited,
    #[error(transparent)]
    RexpectError(#[from] rexpect::error::Error),
}
