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
- `$ program_name [args]` - try to launch `program_name` with `args`
  - `$ ls -l`
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

### Multiprocess example

These instructions may optionally be prefixed with a number (`1$ echo foo`). This allows to launch (and interact with) multiple programs within one test case.
Leaving the number is syntactic sugar for using `0`. 
 
One contrived example may be a program which needs to connect to a server. Then you can launch the server using the test file as well:

```
[Get time from server]
# Start a server
0$ ./server

# Connect to it with the program to verify
1$ ./client 127.0.0.1
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

## This is all very unfinished

Plans are to include some kind of limited variables / placeholders in instructions, a cli, clean up the code, add a license...

This is all very pre-alpha and I mostly published it just to keep myself motivated to work on it :)