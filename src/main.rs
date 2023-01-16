use rexpect;
use rexpect::session::PtySession;
use std::collections::HashMap;
use std::time::Duration;

struct TestRunner {
    timeout: Duration,
    processes: HashMap<u8, PtySession>,
    instructions: Vec<Instruction>,
    variables: HashMap<String, String>,
}

trait TestRunnerInteractions {
    fn new(instructions: Vec<Instruction>) -> TestRunner;
    fn run(self) -> Result<(), rexpect::error::Error>;
    fn set_variables(&self, variables: HashMap<String, String>);
}

impl TestRunnerInteractions for TestRunner {
    fn new(instructions: Vec<Instruction>) -> TestRunner {
        TestRunner {
            timeout: Duration::from_secs(5),
            processes: HashMap::new(),
            instructions: instructions,
            variables: HashMap::new(),
        }
    }

    // TODO: How to handle escape characters?

    fn run(mut self) -> Result<(), rexpect::error::Error> {
        for instruction in &self.instructions {
            match instruction.kind {
                InstructionType::LaunchProcess => {
                    let process =
                        rexpect::spawn(&instruction.payload, Some(self.timeout.as_millis() as u64))?;
                    self.processes.insert(instruction.process_id, process);
                }

                InstructionType::ExpectStdout => {
                    let process = self.processes.get_mut(&instruction.process_id).unwrap();
                    let _ = process.exp_string(&instruction.payload)?;
                    println!("Successfully found '{}'", instruction.payload);
                }

                InstructionType::PutStdin => {
					// TODO: own error types
                    let process = self.processes.get_mut(&instruction.process_id).ok_or(rexpect::error::Error::EmptyProgramName)?;
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

struct Instruction {
    kind: InstructionType,
    process_id: u8,
    payload: String,
}

enum InstructionType {
    LaunchProcess,        // $
    PutStdin,             // <
    ExpectStdout,         // >
    ExpectRegex,          // ~
    SendControlCharacter, // ^
    ExpectExitCode,       // ?
    SetTimeout,           // t
    SetVariable,          // =
}

fn main() {
    let test_instructions = vec![
        Instruction {
            kind: InstructionType::LaunchProcess,
            process_id: 0,
            payload: "uname".into(),
        },
        Instruction {
            kind: InstructionType::ExpectStdout,
            process_id: 0,
            payload: "Linux".into(),
        },
    ];

    let runner = TestRunner::new(test_instructions);
    let res = runner.run();
}
