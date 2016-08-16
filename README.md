Rule
====

About
-----
The Rule API is a parser combinator written in Rust which offers infinite look ahead scanning.
You parse a UTF-8 string into a generic T. Where T is a custom defined data structure for example an AST or a primitive f64. 

You can use Rule for:
* Creating a programming language syntax and parse an AST out of it.
* Making different parsers for different Unicode text based file formats.
* Writing a calculator with correct operator precedence with a few lines of code.
* A text comparer, like a regexp alternative.
* Much more...

This API is used for the Grammer API. But you can use it standalone if that's more your cup-of-thee.

For examples of grammer you can look in the TypeScript version also available in my GitHub account.

License
-------
This project is licensed under the MIT license.