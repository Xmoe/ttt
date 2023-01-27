use crate::common::*;

use expectrl::{self, Signal, WaitStatus};
use nix;
use std::collections::HashMap;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{self, Duration};
use thiserror::Error;

pub struct TestSuiteRunner {
    test_suite: TestSuite,
}

impl TestSuiteRunner {
    pub fn new(test_suite: TestSuite) -> Self {
        TestSuiteRunner { test_suite }
    }

    pub fn run(self) {
        let mut successes: u32 = 0;
        let max_cases = self.test_suite.test_cases.len();

        for test_case in self.test_suite.test_cases {
            let test_runner = TestRunner::new(test_case.clone(), &self.test_suite.variables);
            let text = format!("Running [{}]:", test_case.name);
            let text_offset = 50;
            print!("{:<text_offset$}", text);
            io::stdout().flush().unwrap();
            match test_runner.run() {
                Ok(()) => {
                    println!("Success!");
                    successes += 1;
                }
                Err(e) => println!("FAIL with {:?}", e),
            }
        }
        println!("{}/{} Successfull tests.", successes, max_cases)
    }
}

pub struct TestRunner<'a> {
    test_case: TestCase,
    timeout: Duration,
    processes: HashMap<u8, expectrl::Session>,
    variables: &'a HashMap<String, String>,
}

impl<'a> TestRunner<'a> {
    pub fn new(test_case: TestCase, variables: &'a HashMap<String, String>) -> Self {
        TestRunner {
            // TODO: Hardcoded timeout
            timeout: Duration::from_secs(3),
            processes: HashMap::new(),
            test_case,
            variables,
        }
    }

    pub fn run(mut self) -> Result<(), TestRunnerError> {
        for instruction in self.test_case.instructions {
            /*
            let process = match instruction {
                Instruction::SendStdin { process_id, .. } |
                Instruction::ExpectStdout { process_id, .. } |
                Instruction::ExpectRegex { process_id, .. } |
                Instruction::SendControlChar { process_id, .. } |
                Instruction::ExpectExitCode { process_id, .. } => {
                    let process = self
                    .processes
                    .get_mut(&process_id)
                    .ok_or(TestRunnerError::InvalidProcess)?;
                    Some(process)
                }
                Instruction::LaunchProcess { process_id, .. } => None,
            };
            */

            match instruction {
                Instruction::LaunchProcess{variable, mut string, process_id} => {

                    if let Some(var) = variable {
                        let value = self.variables.get(&var).ok_or(TestRunnerError::UninitializedVariable)?;
                        string = format!("{value} {string}")
                    }

                    match expectrl::spawn(string) {
                        // Transform IO error to own error type
                        Err(expectrl::Error::IO(_)) => return Err(TestRunnerError::ProgramDoesNotExist),
                        session => self.processes.insert(process_id, session?)
                    };
                }

                Instruction::ExpectStdout{string, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    let _ = process.expect(&string)?;
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
                    process.expect(expectrl::Regex(&string))?;
                }

                Instruction::SendControlChar{character, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;
                    process.send_control(character)?;
                }

                Instruction::ExpectExitCode{exit_code, process_id, modifier} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;

                    let expected_exit_code = exit_code;

                    // Emulating the the timeout behaviour from expectrl::session::sync_session::Session::expect_greedy
                    // expectrl timeouts only apply on each Session::expect()
                    let start = time::Instant::now();

                    // Wait until the process has exited or the timeout has been reached
                    let exit_code = loop {
                        match process.status()? {
                            WaitStatus::Exited(_, exit_code) => {
                                break exit_code
                            },

                            WaitStatus::StillAlive |
                            WaitStatus::Continued(_) => {
                                if start.elapsed() > self.timeout {
                                    return Err(TestRunnerError::Timeout)
                                }
                            },

                            // Warn especially on segfault
                            WaitStatus::Signaled(_, Signal::SIGSEGV, _) => return Err(TestRunnerError::SegFault),
                            _ => return Err(TestRunnerError::ProgramStoppedEarly),
                        }
                        // Hardcoded sleep to waste less CPU Cycles
                        sleep(Duration::from_millis(100))
                    };

                    let valid_exit_code = match modifier {
                        ExitCodeModifier::Equals => exit_code == expected_exit_code,
                        ExitCodeModifier::LessThan => exit_code < expected_exit_code,
                        ExitCodeModifier::MoreThan => exit_code > expected_exit_code,
                    };

                    if valid_exit_code {
                        return Ok(())
                    }

                    return Err(TestRunnerError::WrongExitCode)
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
    #[error("The program does not exist or is not executable")]
    ProgramDoesNotExist,
    #[error("Invalid Process ID")]
    InvalidProcess,
    #[error("Wrong exit code")]
    WrongExitCode,
    #[error("Program exited early")]
    ProgramStoppedEarly,
    #[error("Program segfaulted")]
    SegFault,
    #[error("Invalid Control Character")]
    InvalidControlChar,
    #[error("Program timed out")]
    Timeout,
    #[error("Variable not set")]
    UninitializedVariable,
    #[error(transparent)]
    ExpectrlError(#[from] expectrl::Error),
    #[error(transparent)]
    NixError(#[from] nix::Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
}
