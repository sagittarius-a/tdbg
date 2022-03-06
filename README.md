# tdbg

Quick and dirty way to trace code execution and count associated hits.

# Use-case
You're debugging a piece of code and you want to determine if a piece
of code is hit, and if so, how many times it is hit.

`tdbg` provides the `tdbg!` macro to associate an identifier to a code
location. Using the macro will send a message to the `tdbg` server
(standalone binary), that will report every hit and the associated
number of time it has been hit.

Simply fire up the server, insert `tdbg!` macros and you're done.

# Philosophy

- `tdbg` tries to interrupt the original workflow as less as possible
- `tdbg` should never panic in order to not disrupt the origin code
- `tdbg` tries to stay as minimal and portable as possible
- This is not really production ready. Use it at your own risk. Keep
  in mind that it is a tool I personally use and I do not expect
  anyone else to do as well.