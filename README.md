# TTT - Terminal Testing Tool

The terminal testing tool aims to provide a way to check the behaviour of a program as a blackbox.
It has been developed to verify student solutions to programming tasks in a simple way, by emulating inputs of a user and verifying the output matches what is expected for the given input.

These interactions are written in `.test` files in a human readable format.

## File format

The tool reads `test cases` containing `instructions` to execute on programs from `.test` files. These instructions are then executed as if a person was interacting with the program.

### Basic example
```
[Test 1]
$ echo foo
> foo
? 0
```
defines a test case named `Test 1`

which launches the program `echo` with argument `foo`

expects `foo` as output

and expects `0` as exit code from echo

A file may contain multiple test cases:
```
[Test 1]
$ echo foo
> foo
? 0

[Test 2]
$ uname
r (l|L)inux
? 0
```

### Instructions

All instructions need to be assigned to a `test case`, or more specifically listed under a header `[test_case_name]`.

- `# comment` - lines starting with `#` are ignored by the program
- `$ [OPT var] command` - try to launch `command`. If `var` is set, it is replaced by its value when executing the tests
  - `$ ls -l`
  - `$ {var} arg1`
- `> output_string` - expect the program to output exactly `output_string`
  - `> .rw-r--r-- moe moe 490 B  Mon Jan 23 17:54:03 2023 Cargo.toml`
- `r regex` - expect the program to output someting that matches `regex`
  - `r (fine|bad|okay)`
- `< input_data` - write `input_data` to stdin of the program
  - `< hey program, how are you doing?`
- `^ char` - sends a [control character](https://en.wikipedia.org/wiki/C0_and_C1_control_codes) to the process
  - `^ C`
- `? [OPT comparison] exit_code` - expect the program to end with `exit_code`. `comparison` may be `<` or `>` to specify a number above or below a certain value instead of equal value
  - `? <3`


### Variables

Variables can be set in front of the first test case or from the command line when executing ttt. A variable set in the `.test` file will be overwritten by a variable passed to as arg to ttt

```
# This is the first line of the file
{program_name} = echo

[Test Variable]
$ {program_name} blub
> blub
? 0
```

Variables can only be placed in the launch instruction (`$`). This limitation is mainly to keep the tool more simple, while it allows to use the same test file for multiple programs with different names.

### Multiprocess example

These instructions may optionally be prefixed with a number (`1$ echo foo`). This allows to launch (and interact with) multiple programs within one test case.
Leaving the number is syntactic sugar for using `0`. 
 
One contrived example may be a program which needs to connect to a server. Then you can launch the server using the test file as well:

```
{server_path} = testing_tools/server

[Get time from server]
# Start a server
0$ {server_path} 1337

# Connect to it with the program to verify
1$ {program_name} 127.0.0.1:1337
0> Client connected
1> Connected to server

# Send a request to the server
1< time
0> Client requested time
1> 17:54

# End the session
1^ D
1> Goodbye!
0> Client disconnected
0^ C
1? 0
```

Note that we reference the variable `{program_name}` but dont set it. This means it must be supplied as argument to the testing tool.

## This is all very unfinished

This is all very pre-alpha and I mostly published it just to keep myself motivated to work on it :)

### Todo:
- add CLI
- clean up processes at the end of each test case
- add timeout instruction
- add a license
- clean up the code
  - rewrite the parser for the MCCCXXXVIth time
