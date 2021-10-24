// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
  IResult,
  branch::alt,
  combinator::opt,
  multi::{many1, many0, many_till, separated_list},
  bytes::complete::{tag, take, take_until, take_till},
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
  Ok((input, Node::Bool{ value: boolv}))
}

pub fn string(input: &str) -> IResult<&str, Node> {
  let (input, result) = char('\"')(input)?; // eat the first quote
  let (input, result) = take_until("\"")(input)?; // parse through everything except the quote
  let (input, na) = char('\"')(input)?; // eat the last quote, na stores unecessary result
  Ok((input, Node::String{ value: result.to_string()}))
}

pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, result) = take_until("(")(input)?;
  let (i, r2) = identifier(result)?;
  let fn_name = String::from(result.clone());
  let (input, result) = char('(')(input)?;
  let (input, args) = alt((other_arg, arguments))(input)?;
  let (input, result) = char(')')(input)?;
  Ok((input, Node::FunctionCall{ name: fn_name, children: vec![args]}))
}

pub fn arguments(input: &str) -> IResult<&str, Node> {
  //println!("slice[0] = {:?}", input.bytes().next());
  let i = input.bytes().next().unwrap();
  if i == (')' as u8) { // no arguments
    Ok((input, Node::FunctionArguments{children: vec![]}))
  } else {
    let (input, result) = alt((identifier, math_expression))(input)?;
    let fun_arg = Node::FunctionArguments{children: vec![result]};
    Ok((input, fun_arg))
  }
}

// Like the first argument but with a comma in front
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  let (input, args) = take_until(")")(input)?;
  let split = args.split(",");
  let mut ch: Vec<Node> = vec![];
  for a in split {
    let (i, r) = alt((math_expression, number, identifier))(a)?;
    ch.push(Node::Expression{children: vec![r]});
  }
  
  Ok((input, Node::FunctionArguments{children: ch}))
}

// Math expressions with parens ((10+2)*6)/4
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  let mut ch: Vec<Node> = vec![];
  let (input, result) = tag("(")(input)?;
  let (input, args) = math_expression(input)?;
  let (input, result) = tag(")")(input)?;
  Ok((input, Node::Expression{children: vec![args]}))
}

pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((function_call, number, identifier, parenthetical_expression))(input)
}

pub fn l3_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = tag("^")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l3(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}

pub fn l3(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l4(input)?;
  let (input, tail) = many0(l3_infix)(input)?;
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

pub fn l2_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("*"),tag("/")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l3(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}

pub fn l2(input: &str) -> IResult<&str, Node> {
  let (input, mut head) = l3(input)?;
  let (input, tail) = many0(l2_infix)(input)?;
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
  println!("Doing math");
  l1(input)
}

pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((string, boolean, function_call, math_expression, number, identifier))(input)?;
  let realResult = Node::Expression{children: vec![result]};
  Ok((input, realResult))
}

pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag("\n"))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, result) = variable_define(input)?;
  let (input, _) = tag(";")(input)?;
  
  Ok((input, Node::Statement{children: vec![result]}))
}

pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag("\n"))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, result) = tag("return ")(input)?;
  let (input, return_val) = take_until(";")(input)?;
  let (i, ident) = alt((number, function_call, expression, identifier))(return_val)?;
  let (input, _) = tag(";")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = many0(tag("\n"))(input)?;
  Ok((input, Node::FunctionReturn{children: vec![ident]}))
}

// Define a statement of the form
// let x = expression*/
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("let ")(input)?;
  let (input, variable) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("=")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, expr) = take_until(";")(input)?;
  let (i, expression) = expression(expr)?;
  Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
}
pub fn function_definition(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("fn ")(input)?;
  let (input, fn_name) = take_until("(")(input)?;
  let (_, func_name) = identifier(fn_name)?;
  let (input, _) = tag("(")(input)?;
  let (input, args) = alt((other_arg, arguments))(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, mut stats) = many0(alt((comment, statement)))(input)?;
  let (input, fn_return) = function_return(input)?;
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(tag("\n"))(input)?;
  let mut v_def = vec![];
  for i in 0..stats.len() {
    match stats[i].clone() {
      Node::Statement{children} => {
        for j in 0..children.len() {
          v_def.push(children[j].clone());
        }
      }
      _ => {
        println!("Nothing found here");
      }
    }
  }
  let func_stats = Node::FunctionStatements{children: v_def.clone()};
  let mut return_vec = vec![func_name, args];
  for i in 0..v_def.len() {
    return_vec.push(Node::Statement{children: vec![v_def[i].clone()]});
  }
  return_vec.push(fn_return);
  Ok((input, Node::FunctionDefine{children: return_vec}))


}

pub fn comment(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag("\n"))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("//")(input)?;
  let (input, _) = take_until("\n")(input)?;
  let (input, _) = tag("\n")(input)?;
  Ok((input, Node::Statement{children: vec![]})) // return empty statement for comment
}

// Define a program. You will change this, this is just here for example.
// You'll probably want to modify this by changing it to be that a program
// is defined as at least one function definition, but maybe more. Start
// by looking up the many1() combinator and that should get you started.
pub fn program(input: &str) -> IResult<&str, Node> {
    let (input, result) = many0(alt((function_definition, statement, expression)))(input)?;  // Now that we've defined a number and an identifier, we can compose them using more combinators. Here we use the "alt" combinator to propose a choice.
    Ok((input, Node::Program{ children: result}))       // Whether the result is an identifier or a number, we attach that to the program
  
}
