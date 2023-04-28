# this file is responsible for displaying the results of a test

from dataclasses import dataclass
from ttt_common import TestFailed
from pathlib import Path
from colorama import Back


@dataclass
class TestResults:
    program: str
    results: list[None | TestFailed]

    def display(self):
        failures = [x for x in self.results if x is not None]
        num_successes = len(self.results) - len(failures)

        bg_color = Back.GREEN if len(
            failures) == 0 else Back.RED

        print(
            f"\n{bg_color}[SUMMARY] [{num_successes:2}/{len(self.results):2}] {Path(self.program).stem}{Back.RESET}")
        for failure in failures:
            print(
                f"{Back.LIGHTYELLOW_EX}Test '{failure.test_name}' at line {failure.line_number}:{Back.RESET}")
            print(
                f"  Expected: {Back.LIGHTGREEN_EX}{failure.expected_value}{Back.RESET}")
            print(
                f"  Actual:   {Back.YELLOW}{failure.actual_value}{Back.RESET}")
