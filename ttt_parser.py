from typing import List
from dataclasses import dataclass
from datetime import timedelta
from enum import Enum, auto
import re
from pprint import pprint

instruction_regex = re.compile(
    r" *(?P<process_id>\d*)(?P<instruction_kind>\$|<|>|r|^|\?) (?P<payload>.*)")
test_header_regex = re.compile(r"\[(?P<test_name>.*)\]")


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
    original_line: int


@dataclass
class SingleTestCase:
    name: str
    instructions: List[Instruction]
    #timeout: timedelta = timedelta(milliseconds=500)


@dataclass
class TestSuite:
    name: str
    tests: List[SingleTestCase]


def parse(file) -> TestSuite:
    with open(file) as file:

        # create incomplete prototype of testsuite and forward-declare test_case and instruction
        test_suite = TestSuite(file.name, [])
        test_case = None
        instruction = None

        for index, line in enumerate(file):
            # Skip newlines and comments
            if line.startswith("#") or line.isspace():
                continue

            elif match := re.match(test_header_regex, line):
                if test_case is not None:
                    test_suite.tests.append(test_case)
                # create incomplete prototype of test_case
                test_case = SingleTestCase(match.group("test_name"), [])

            elif match := re.match(instruction_regex, line):
                parts = match.groupdict()
                # set the default process id for syntactic sugar
                if parts["process_id"] == "":
                    parts["process_id"] = 0
                parts["process_id"] = int(parts["process_id"])

                instruction = None
                match parts["instruction_kind"]:
                    case "$": instruction = Instruction(parts["process_id"], InstructionKind.LaunchProcess, parts["payload"], index)
                    case "<": instruction = Instruction(parts["process_id"], InstructionKind.SendStdin, parts["payload"], index)
                    case ">": instruction = Instruction(parts["process_id"], InstructionKind.ExpectStdout, parts["payload"], index)
                    case "r": instruction = Instruction(parts["process_id"], InstructionKind.RegexStdout, parts["payload"], index)
                    case "?": instruction = Instruction(parts["process_id"], InstructionKind.ExpectExitCode, parts["payload"], index)
                    case "^": instruction = Instruction(parts["process_id"], InstructionKind.SendControlChar, parts["payload"], index)

                test_case.instructions.append(instruction)


            else:
                print(f"invalid line: {index+1}")
                exit(-1)
        # after the loop we still need to put the partially parsed objects into their parents
        if test_case is not None:
            test_suite.tests.append(test_case)

        pprint(test_suite)
