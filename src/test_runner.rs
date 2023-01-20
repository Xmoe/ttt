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
            print!("Running Test {}: {}\t", index + 1, test_case.name);
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
            let process_id = instruction.process_id;
            match instruction.specialization {
                InstructionType::LaunchProcess(payload) => {

                    // rexpect gives no way to check whether a process has been successfully created until something is expected :(
                    // Use rust-psutil to detect aliveness of the program?

                    let process = rexpect::session::spawn(&payload, Some(self.timeout.as_millis() as u64))?;
                    if let Some(WaitStatus::StillAlive) = process.process.status() {
                        self.processes.insert(process_id, process);
                    } else {
                        return Err(TestRunnerError::InvalidProcess)
                    };
                }

                InstructionType::ExpectStdout => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    let _ = process.exp_string(&payload)?;
                }

                InstructionType::PutStdin => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_line(&payload)?;
                }

                InstructionType::ExpectRegex => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.exp_regex(&payload)?;
                }

                InstructionType::SendControlCharacter => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_control(payload.chars().nth(0).unwrap())?;
                }

                InstructionType::ExpectExitCode => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;


                    //TODO: maybe set default timeout again for safety, because wait is blocking!
                    if let Ok(WaitStatus::Exited(_, exit_code)) = process.process.wait() {
                        println!("Checking exit code");
                        let expected_exit_code = i32::from_str_radix(&payload, 10).unwrap();
                        if exit_code != expected_exit_code {
                            println!("Exit codes are unequal!");
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
