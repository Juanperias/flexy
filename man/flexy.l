.TH FLEXY 1 "20 Sep 2024" "1.0" "Flexy Docs"
.SH NAME
flexy \- an extremely fast and easy to use widget composer
.SH DESCRIPTION
flexy is an extremely fast and easy to use widget composer. To create widgets you use Lua, so we will talk in detail about the functions to interact with flexy.
.SH UI FUNCTIONS
.B text
renders a text on screen
.PP
Example:
.EX
text("Hello!")
.EE
Arguments:
.IP \[bu] 2
message: String
.SH LOG FUNCTIONS
.B debug
displays a message when loading the Lua
.PP
Example:
.EX
debug("Hello!")
.EE
Arguments:
.IP \[bu] 2
message: String
.B error
displays an error message and closes the program
.PP
Example:
.EX
error("Oh no")
.EE
Arguments:
.IP \[bu] 2
message: String

