# lispers
A slightly corroded LISP interpreter.

There are two things want in life, those are:
* to learn the rust programming language
* to write a LISP interpreter

So I did the obvious thing and started on an interpreter written in rust.

After working on this for a while I now have the following (v0.1):
* A working interpreter capable of basic integer arithmetics and nested expressions
* Various functions mainly operating on integers
* Definition of global vars
* Lambda expressions and functions as a data type
* A bunch of tests to keep it stable when proceeding

Next up (v0.2):
* data-mode, in other words: lists that are not function calls (this exists through lambda already)
* As a result of the previous point: the map function, implemented through lisp.
* Floating point numbers, I am treading carefully here so I can map out a clear typing hierarchy.
* Several sections of the code are just incredibly messy, this needs to be fixed.

My end goal:
* A fully functional interpreter with support for various data types and structures.
* An interpreter capable of interpreting a meta-circular interpreter (emulating itself).
* A project that reflects a solid understanding of rustlang. (This is not the case at the moment!) 
