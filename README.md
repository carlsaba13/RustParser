# CSE262 - Programming Languages - Homework 7b

**Due Date: 10/25/2021 by EOD**

**This assignment is out of 100 points. All tests are weighted equally.**

## Instructions

1. Fork this repository into your CSE262 project namespace. [Instructions](https://docs.gitlab.com/ee/workflow/forking_workflow.html#creating-a-fork)
2. Clone your newly forked repository onto your development machine. [Instructions](https://docs.gitlab.com/ee/gitlab-basics/start-using-git.html#clone-a-repository) 
3. As you are writing code you should commit patches along the way. *i.e.* don't just submit all your code in one big commit when you're all done. Commit your progress as you work. You should have at least one commit per question. **The assignment will not be accepted if you do not follow this procedure.**
4. When you've committed all of your work, there's nothing left to do to submit the assignment.

## Assignment

In this project you will write a grammar and a parser that recognizes the grammar. I've provided a number of statements in the language in `src/parser.rs`, which runs tests against your code. There are 28 tests you will need to pass to get full credit. You can think of each test as a question. If you write code that passes more than one test, just make a dummy commit to satisfy the commit requirement.

An example program in the language looks like this:

```
fn foo(a,b,c) {
  let x = a + 1; 
  // This is a comment
  let y = bar(c - b);
  return x * y; // Multiply the results
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}
```

This is almost the full language. You can write functions, call functions, do math, and return values. Strings and Booleans are also data types in the language, but the runtime doesn't do anything with them. See `tests/parser.rs` for more example strings in this language.

The output of your parser is a tree of Nodes. You can find a list of Node types in `src/parser.rs` The tree should contain a single root "Program" node. The rest of the tree you will have to determine using the test cases, as well as the runtime found in `src/runtime.rs` (you won't have to modify this file). You can use this runtime to reverse engineer the parser in a sense. *i.e.* the runtime works on the result of the parser, so whatever your parser produces it needs to be in a form the runtime works with.

## Parser Combinators

We will be using the [nom](https://crates.io/crates/nom) parser combinator library in Rust. A bit about combinators:

Parser combinators are an approach to parsers that is very different from software like lex and yacc. Instead of writing the grammar in a separate syntax and generating the corresponding code, you use very small functions with a very specific purpose, like "take 5 bytes", or "recognize the word 'HTTP'", and assemble then in meaningful patterns like "recognize 'HTTP', then a space, then a version". The resulting code is small, and looks like the grammar you would have written with other parser approaches.

This gives us a few advantages:

- the parser is small and easy to write
- the parser's components are easy to reuse (if they're general enough, please add them to nom!)
- the parser's components are easy to test separately (with unit tests as in `/tests/parser.rs`)
- the parser combination code looks close to the grammar you would have written
- you can build partial parsers, specific to the data you need at the moment, and ignore the rest
- Here is an example of one such parser, to recognize text between parentheses:

```rust
use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::char,
  bytes::complete::is_not
};

fn parens(input: &str) -> IResult<&str, &str> {
  delimited(char('('), is_not(")"), char(')'))(input)
}
```

It defines a function named `parens` which will recognize a sequence of the character `(`, the longest byte array not containing `)`, then the character `)`, and will return the byte array in the middle.

Here is another parser, written without using nom's combinators this time:

```rust
#[macro_use]
extern crate nom;

use nom::{IResult, Err, Needed};

fn take4(i: &[u8]) -> IResult<&[u8], &[u8]>{
  if i.len() < 4 {
    Err(Err::Incomplete(Needed::Size(4)))
  } else {
    Ok((&i[4..], &i[0..4]))
  }
}
```

This function takes a byte array as input, and tries to consume 4 bytes. Writing all the parsers manually, like this, is dangerous, despite Rust's safety features. There are still a lot of mistakes one can make. That's why nom provides a list of function and macros to help in developing parsers.

With nom, you would write it like this:

```rust
use nom::{IResult, bytes::streaming::take};
fn take4(input: &str) -> IResult<&str, &str> {
  take(4u8)(input)
}
```

A parser in nom is a function which, for an input type I, an output type O and an optional error type E, will have the following signature:

```rust
fn parser(input: I) -> IResult<I, O, E>;
```

Or like this, if you don't want to specify a custom error type (it will be u32 by default):

```rust
fn parser(input: I) -> IResult<I, O>;
```

IResult is an alias for the Result type:

```rust
use nom::{Needed, error::ErrorKind};

type IResult<I, O, E = (I,ErrorKind)> = Result<(I, O), Err<E>>;

enum Err<E> {
  Incomplete(Needed),
  Error(E),
  Failure(E),
}
```

It can have the following values:

- a correct result Ok((I,O)) with the first element being the remaining of the input (not parsed yet), and the second the output value;
- an error Err(Err::Error(c)) with c an error that can be built from the input position and a parser specific error
- an error Err(Err::Incomplete(Needed)) indicating that more input is necessary. Needed can indicate how much data is needed
- an error Err(Err::Failure(c)). It works like the Error case, except it indicates an unrecoverable error: we cannot backtrack and test another parser

Please refer to the ["choose a combinator"](https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md) guide for an exhaustive list of parsers. See also the rest of the documentation [here](https://docs.rs/nom/5.0.1/nom/).

## Deliverables

For this assignment produce two deliverables: a language grammar and a parser written using hte nom library.

### Part 1: Grammar

Do the language grammar first. Along with the parser you should write an ebnf grammar for the language in `/grammar.ebnf`. This file already contains the nodes in the grammar you need to implement. Use the EBNF format. Note, if you do this correctly, you'll find the EBNF grammar maps almost 1:1 with the parer combinators you'll write. Furthermore, the semantics of EBNF (such as optional nodes, repeated nodes, choice nodes, etc.) map directly to combinators provided by `nom`. You'll find even more options in the reference linked above.

To write this grammar, you should consult the tests in `/tests/parser.rs`. Each test represents a string in the language, so your grammar should recognize each one of these. You will also need to look at the runtime in `/src/runtime.rs`. The runtime takes a parse tree produced by the parser and executes it using a stack machine. By looking at how the runtime works, you will be able to reverse engineer what the parse tree should look like. 

### Part 2: Parser

Finish the parser that's started in `/src/parser.rs`. It should implement the grammar that you wrote in Part 1. The input is passed into the function `program()` as a `&str`, and that kicks off the parsing process. The way program is currently defined, it states that a program is either a number or an identifier. Of course, this is not correct, as a program is more complicated than that, so your solution will have to modify the `program()` function to match the language grammar. `number` and `identifier` are the names of other parser combinators, each which conforms to the same function signature: 

```rust
pub fn combinator_name(input: &str) -> IResult<&str, Node>
```

The file defines two other combinators `number()` and `identifier()`. You'll need to write other combinators for the various other nodes in the grammar, and tie them together using the various parser combinators that are included in the nom library. You can find a list of the built-in combinators and what they do [here](https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md).

Here is an example combinator that recognizes a variable of the form:

`let <identifier> = <expression>;`

```rust
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expression) = expression(input)?;
  Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
}
```

The parser is automatically tested in `tests/parser.rs`. Each string is processed by the parser, and the resulting parse tree is fed into the runtime and executed, producing a result. There are 28 different statements in the language along with their expected output from the runtime. In order to get full credit, you'll need to pass all 28 tests.

