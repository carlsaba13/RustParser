// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
  IResult,
  branch::alt,
  combinator::opt,
  multi::{many1, many0},
  bytes::complete::{tag, take, take_until},
  character::complete::{alphanumeric1, digit1, char},
  sequence::separated_pair,
};

// Here are the different node types. You will use these to make your parser and your grammar.
// You may add other nodes as you see fit, but these are expected by the runtime.

#[derive(Debug, Clone)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
}

// Define production rules for an identifier
pub fn identifier(input: &str) -> IResult<&str, Node> {
  println!("In identifier");
  let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
  Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
}

// Define an integer number
pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
  let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
  Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
}

pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((tag("true"), tag("false")))(input)?;
  let boolv = result.parse::<bool>().unwrap();
  println!("boolv = {:?}", boolv);
  Ok((input, Node::Bool{ value: boolv}))
}

pub fn string(input: &str) -> IResult<&str, Node> {
  let (input, result) = char('\"')(input)?; // eat the first quote
  let (input, result) = take_until("\"")(input)?; // parse through everything except the quote
  let (input, na) = char('\"')(input)?; // eat the last quote, na stores unecessary result
  Ok((input, Node::String{ value: result.to_string()}))
}

pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, result) = identifier(input)?;
  let fn_name = String::from(input.clone());
  println!("Input = {:?}", input);
  let (input, result) = char('(')(input)?;
  let (input, args) = take_until(")")(input)?;
  let s = args.split(",");
  let mut child: Vec<Node> = vec![];
  let ch: Vec<&str> = s.collect();
  println!("ch.len = {:?}", ch.len());
  println!("ch = {:?}", ch);
  if ch.len() > 1 {
    for c in ch.iter() {
      let (i, n) = identifier(c)?;
      assert_eq!(i, "");
      child.push(n);
    }
  }
  
  
  let (input, result) = char(')')(input)?;
  Ok((input, Node::FunctionCall{ name: fn_name, children: child}))
}

// Math expressions with parens (1 * (2 + 3))
/*pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((function_call, number, identifier, parenthetical_expression))(input)
}

pub fn l3_infix(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn l3(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn l2_infix(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn l2(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// L1 - L4 handle order of operations for math expressions 
pub fn l1_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("+"),tag("-")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l2(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}

pub fn l1(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l2(input)?;
  let (input, tail) = many0(l1_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => () 
    };
  }
  Ok((input, head))
}

pub fn math_expression(input: &str) -> IResult<&str, Node> {
  l1(input)
}

pub fn expression(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn statement(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn function_return(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// Define a statement of the form
// let x = expression
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expression) = expression(input)?;
  Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
}

pub fn arguments(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// Like the first argument but with a comma in front
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn function_definition(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn comment(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}*/

// Define a program. You will change this, this is just here for example.
// You'll probably want to modify this by changing it to be that a program
// is defined as at least one function definition, but maybe more. Start
// by looking up the many1() combinator and that should get you started.
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((function_call, number, boolean, identifier, string))(input)?;  // Now that we've defined a number and an identifier, we can compose them using more combinators. Here we use the "alt" combinator to propose a choice.
  let realResult = Node::Expression{children: vec![result]};
  //println!("realResult = {:?}", realResult);
  Ok((input, Node::Program{ children: vec![realResult]}))       // Whether the result is an identifier or a number, we attach that to the program
}
