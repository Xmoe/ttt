from copy import deepcopy
from dataclasses import dataclass
from time import sleep

import pexpect

from ttt_common import *

# TODO: get timeout from test file and store it in SingleTestCase
TIMEOUT = 2


@dataclass
class TestSuiteRunner:
    test_suite: TestSuite
    variables: dict[str, str]

    def run(self):
        # deepcopy the test_case, because when the variables are replaced, following runs would use the same replaced variable
        return [VirtualMachine(deepcopy(test_case), self.variables).run() for test_case in self.test_suite.tests]


@dataclass
class VirtualMachine:
    """
    This class contains the references to processes started for running a single test
    """

    test_case: SingleTestCase
    variables: dict[str, str]

    def run(self) -> TestResult:
        """
        Returns either a TestResult with successfull=True
        or successfull=False and context information about what went wrong
        """
        self.processes = dict()

        # apply variables
        for index, instruction in enumerate(self.test_case.instructions):
            for (key, value) in self.variables.items():
                payload = instruction.payload.replace(key, value)
                self.test_case.instructions[index].payload = payload

        for instruction in self.test_case.instructions:
            match instruction.kind:

                case InstructionKind.LaunchProcess:
                    cmdline = pexpect.split_command_line(instruction.payload)
                    cmd, args = cmdline[0], cmdline[1:]
                    try:
                        child = pexpect.spawn(cmd, args)
                        self.processes[instruction.process_id] = child
                    except pexpect.ExceptionPexpect as e:
                        if "The command was not found or was not executable:" in e.value:
                            return TestResult(False, self.test_case.name, "Process spawned", f"Process {instruction.payload} could not be spawned", instruction.line_number)
                        raise e

                case InstructionKind.SendStdin:
                    process = self.processes[instruction.process_id]
                    process.send(instruction.payload)

                case InstructionKind.SendControlChar:
                    process = self.processes[instruction.process_id]
                    process.sendcontrol(instruction.payload)

                case InstructionKind.ExpectStdout:
                    process = self.processes[instruction.process_id]
                    try:
                        process.expect_exact(
                            instruction.payload, timeout=TIMEOUT)
                    except pexpect.EOF:
                        return TestResult(False, self.test_case.name, instruction.payload, process.before, instruction.line_number)
                    except pexpect.TIMEOUT:
                        return TestResult(False, self.test_case.name, instruction.payload, "[PROCESS TIMED OUT]", instruction.line_number)

                case InstructionKind.RegexStdout:
                    process = self.processes[instruction.process_id]
                    try:
                        process.expect(instruction.payload, timeout=TIMEOUT)
                    except pexpect.EOF:
                        return TestResult(False, self.test_case.name, instruction.payload, process.before, instruction.line_number)
                    except pexpect.TIMEOUT:
                        return TestResult(False, self.test_case.name, instruction.payload, "[PROCESS TIMED OUT]", instruction.line_number)

                case InstructionKind.ExpectExitCode:
                    process = self.processes[instruction.process_id]
                    if process.isalive():
                        sleep(2)
                    if process.isalive():
                        # TODO maybe timeout
                        process.terminate(force=True)
                    if process.exitstatus == int(instruction.payload):
                        # this indicates a success
                        return TestResult(True, self.test_case.name)
                    else:
                        return TestResult(False, self.test_case.name, instruction.payload, process.exitstatus, instruction.line_number)

        return TestResult(True, self.test_case.name)
