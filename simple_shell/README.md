### Very bearbones shell

Idea behind this was to learn how easy it acctualy is to make something like this

There are multiple ways to improve on this project:

1. Instead of using Command and Process from Rust, include syscallscrate and do it with fork and execv systemcalls
2. Implement more builtins and more operations that could be done with CLI, make this CLI a viable daily driver
3. Instead of using Command that allows us to use Stdio::inherit and Stdio::piped so underlying programs are aware what they should do, make our own Command and do the
standard input, ouput, arguments and piping ourselves.


All credit to: joshmcguigan
