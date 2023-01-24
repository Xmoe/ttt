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

    // TODO: Add context like programs vars etc to parameters
    pub fn run(self) {
        let mut successes: u32 = 0;
        let max_cases = self.test_suite.test_cases.len();

        for test_case in self.test_suite.test_cases {
            let test_runner = TestRunner::new(test_case.clone());
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

pub struct TestRunner {
    timeout: Duration,
    processes: HashMap<u8, expectrl::Session>,
    test_case: TestCase,
}

impl TestRunner {
    pub fn new(test_case: TestCase) -> Self {
        TestRunner {
            // TODO: Hardcoded timeout
            timeout: Duration::from_secs(3),
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
                    let mut session = expectrl::spawn(string)?;
                    session.set_expect_timeout(Some(self.timeout));

                    self.processes.insert(process_id, session);

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

                Instruction::ExpectExitCode{exit_code, process_id} => {
                    let process = self
                        .processes
                        .get_mut(&process_id)
                        .ok_or(TestRunnerError::InvalidProcess)?;

                    let expected_exit_code = exit_code;

                    // Emulating the the timeout behaviour from expectrl::session::sync_session::Session::expect_greedy
                    // expectrl timeouts only apply on each Session::expect()
                    let start = time::Instant::now();

                    // Wait until the process has exited or the timeout has been reached
                    loop {
                        match process.status()? {
                            WaitStatus::Exited(_, exit_code) => {
                                if exit_code == expected_exit_code {
                                    return Ok(())
                                } else {
                                    return Err(TestRunnerError::WrongExitCode)
                                }
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
    #[error("Program exited early")]
    ProgramStoppedEarly,
    #[error("Program segfaulted")]
    SegFault,
    #[error("Invalid Control Character")]
    InvalidControlChar,
    #[error("Program timed out")]
    Timeout,
    #[error(transparent)]
    ExpectrlError(#[from] expectrl::Error),
    #[error(transparent)]
    NixError(#[from] nix::Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
}
