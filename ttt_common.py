from dataclasses import dataclass
from enum import Enum, auto
from pathlib import Path

from colorama import Back

from pprint import pprint


class InstructionKind(Enum):
    LaunchProcess = auto()
    SendStdin = auto()
    ExpectStdout = auto()
    RegexStdout = auto()
    SendControlChar = auto()
    ExpectExitCode = auto()


@dataclass
class Instruction:
    process_id: int
    kind: InstructionKind
    payload: bytes
    line_number: int


@dataclass
class SingleTestCase:
    name: str
    instructions: list[Instruction]
    #timeout: timedelta = timedelta(milliseconds=500)


@dataclass
class TestSuite:
    name: str
    tests: list[SingleTestCase]


@dataclass
class TestResult:
    successfull: bool
    test_name: str
    expected_value: str = None
    actual_value: str = None
    line_number: int = None


@dataclass
class TestResults:
    program: str
    results: list[TestResult]

    def display(self):
        pprint(self.results)

        failures = [test for test in self.results if not test.successfull]
        pprint(failures)
        num_successes = len(self.results) - len(failures)

        bg_color = Back.GREEN if len(
            failures) == 0 else Back.RED

        print(
            f"\n{bg_color}[SUMMARY] [{num_successes:2}/{len(self.results):2}] {Path(self.program).stem}{Back.RESET}")
        for failure in failures:
            print(
                f"{Back.LIGHTYELLOW_EX}Test [{failure.test_name}] at line {failure.line_number}:{Back.RESET}")
            print(
                f"  Expected: {Back.LIGHTGREEN_EX}{failure.expected_value}{Back.RESET}")
            print(
                f"  Actual:   {Back.YELLOW}{failure.actual_value}{Back.RESET}")
