#! /usr/bin/python3

import argparse
from pathlib import Path

from ttt_parser import parse
from ttt_runner import TestSuiteRunner
from ttt_common import TestResults

cli = argparse.ArgumentParser(
    description="This program reads instructions from .test files and executes them on other processes as if they were interacted with by a human.",
    epilog="This software only took like 5 attempts :)"
)

cli.add_argument("test_vector",
                 help="the path to the .test file",
                 type=Path)
cli.add_argument("test_programs",
                 help="a list of programs to apply the .test files to",
                 type=Path,
                 nargs="+")

args = cli.parse_args()

test_suite = parse(args.test_vector)

# TODO better variable handling

for program in args.test_programs:
    suite_runner = TestSuiteRunner(
        test_suite, {"{0}": str(program.absolute())})
    # TODO change return type of SuiteRunner
    results = suite_runner.run()
    results = TestResults(program, results)
    results.display()
