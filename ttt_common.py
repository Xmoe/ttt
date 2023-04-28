from enum import Enum, auto
from dataclasses import dataclass


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
class TestFailed:
    test_name: str
    expected_value: str
    actual_value: str
    line_number: int
