.TH FLEXY 1 "27 sep 2024" "0.1" "Flexy Manual"
.SH NAME
flexy \- an incredible way to create widgets
.SH SYNOPSIS
flexy is a system (or as I like to call it, composer) of widgets in the style of eww. Flexy is made to be fast and easy to use; you use Lua to create the widgets.

Flexy works as follows:
When you run `flexy start <screen>`, it finds and runs your widget, which is then passed to a struct Job for processing.
.SH MOTIVATION
When I first tried eww, I didn't really understand how to use it because yuck can be a bit more difficult than a language like Lua. Plus, I wanted to make a system that was optimized, fast, and simple.
Flexy is an extremely fast and easy-to-use widget composer. To create widgets, you use Lua, so we will talk in detail about the functions to interact with Flexy.
.SH UI FUNCTIONS
.B text
renders text on the screen
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

