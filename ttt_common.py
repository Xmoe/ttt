from dataclasses import dataclass
from enum import Enum, auto
from pathlib import Path

from colorama import Back, Fore


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
    expected_value: bytes = None
    actual_value: bytes = None
    line_number: int = None


@dataclass
class TestResults:
    program: str
    results: list[TestResult]

    def display(self):
        failures = [test for test in self.results if not test.successfull]
        num_successes = len(self.results) - len(failures)

        bg_color = Back.GREEN if len(
            failures) == 0 else Back.RED

        print(
            f"\n{bg_color}[SUMMARY] [{num_successes:2}/{len(self.results):2}] {Path(self.program).stem}{Back.RESET}")
        for failure in failures:
            print(
                f"{Fore.LIGHTYELLOW_EX}Test [{failure.test_name}] at line {failure.line_number}:{Fore.RESET}")
            print(
                f"  Expected: {Fore.LIGHTGREEN_EX}{failure.expected_value}{Fore.RESET}")
            print(
                f"  Actual:   {Fore.LIGHTRED_EX}{failure.actual_value}{Fore.RESET}")
