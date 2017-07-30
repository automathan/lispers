# lispers
A slightly corroded LISP interpreter.

There are two things want in life, those are:
* to learn the rust programming language
* to write a LISP interpreter

So I did the obvious thing and started on an interpreter written in rust.

After working on this for a while I now have the following (v0.18):
* a working interpreter capable of basic integer arithmetics and nested expressions
* various functions mainly operating on integers
* definition of global vars, local scopes in functions
* lambda expressions and functions as a data type
* a bunch of tests to keep it stable when proceeding
* data-mode, in other words: lists that are not function calls, lists of data
* recursive iteration through lists using car, cdr & cond
* a first draft for an error system (no more printing during interpretation)

Next up (v0.25):
* a nice and clean system for handling the primitive functions
* a content-rich built-in library of functions and global constants etc.
* floating point numbers, I am treading carefully here so I can map out a clear typing hierarchy.
* several sections of the code are just incredibly messy, this needs to be fixed.
* I solve way too many problems by appending .clone() to something, that feels wrong. 

My end goal:
* A fully functional interpreter with support for various data types and structures.
* An interpreter capable of interpreting a meta-circular interpreter (emulating itself).
* A project that reflects a solid understanding of rustlang. (This is not the case at the moment!) 
