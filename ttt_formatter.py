# this file is responsible for displaying the results of a test

from dataclasses import dataclass
from ttt_common import TestFailed

@dataclass
class TestResults:
    program: str
    results: list[None|TestFailed]

    def display(self):
        failures = [x for x in self.results if x is not None]
        successes = len(self.results) - len(failures)
        print(f"\n[SUMMARY] [{successes:2}/{len(self.results):2}] {self.program}")
        for failure in failures:
            print(f"Test '{failure.test_name}' at line {failure.line_number}:")
            print(f"\tExpected: {failure.expected_value}")
            print(f"\tActual:   {failure.actual_value}")