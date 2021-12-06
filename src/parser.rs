// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.
use nom::{
  IResult,
  branch::alt,
  combinator::opt,
  multi::{many1, many0},
  bytes::complete::{tag},
  character::complete::{alphanumeric1, digit1},
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
  If {condition: Vec<Node>, children: Vec<Node>},
  Condition {conditions: Vec<Node>},
  ConditionExpression{name: String, children: Vec<Node>},
  TestEquality {children: Vec<Node>}
}
// Define production rules for an identifier
pub fn identifier(input: &str) -> IResult<&str, Node> {
  let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
  Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
}

/*pub fn n(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((tag("0"),tag("1"),tag("2"),tag("3"),tag("4"),tag("5"),tag("6"),tag("7"),tag("8"),tag("9")))(input)?;
}*/

pub fn binary(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("0")(input)?;
  let (input, _) = tag("b")(input)?;
  let (input, result) = many0(alt((tag("0"), tag("1"))))(input)?;
  let mut s = String::from("");
  for i in result {
    s += i;
  }
  let intval = i32::from_str_radix(&s, 2).unwrap();
  Ok((input, Node::Number{value: intval}))
}

pub fn octal(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("0")(input)?;
  let (input, _) = tag("o")(input)?;
  let (input, result) = many0(alt((tag("0"), tag("1"), tag("2"), tag("3"), tag("4"), tag("5"), tag("6"), tag("7"), tag("8"))))(input)?;
  let mut s = String::from("");
  for i in result {
    s += i;
  }
  let intval = i32::from_str_radix(&s, 8).unwrap();
  Ok((input, Node::Number{value: intval}))
}

pub fn decimal(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn hexidecimal(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn scientific(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn integer(input: &str) -> IResult<&str, Node> {
  alt((integer0, integer1))(input)
}

pub fn integer0(input: &str) -> IResult<&str, Node> {
  let (input, result) = tag("0")(input)?; 
  let number = result.parse::<i32>().unwrap();
  Ok((input, Node::Number{value: number}))
}

pub fn integer1(input: &str) -> IResult<&str, Node> {
  let (input, r1) = alt((tag("1"),tag("2"),tag("3"),tag("4"),tag("5"),tag("6"),tag("7"),tag("8"),tag("9")))(input)?;
  let (input, r2) = many0(digit1)(input)?;
  let mut result = String::from(r1);
  for i in r2 {
    result += i;
  }
  let number = result.parse::<i32>().unwrap();
  Ok((input, Node::Number{value: number}))
}

pub fn floating_point(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// Define an integer number
pub fn number(input: &str) -> IResult<&str, Node> {
  alt((binary, octal, integer))(input)
  //alt((binary, octal, decimal, hexidecimal, scientific, integer, floating_point))(input)
  /*let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
  let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
  Ok((input, Node::Number{ value: number}))*/                 // Return the now partially consumed input with a number as well
}
pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((tag("true"),tag("false")))(input)?;
  let bool_value = if result == "true" {true} else {false};
  Ok((input, Node::Bool{ value: bool_value}))
}
pub fn string(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("\"")(input)?;
  let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
  let (input, _) = tag("\"")(input)?;
  Ok((input, Node::String{ value: string.join("")}))
}
pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, name) = alphanumeric1(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, mut args) = many0(arguments)(input)?;
  let (input, _) = tag(")")(input)?;
  Ok((input, Node::FunctionCall{name: name.to_string(), children: args}))   
}
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l1(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  Ok((input, args))
}
pub fn parenthetical_condition(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = condition(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  Ok((input, args))
}

pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((function_call, number, identifier, parenthetical_expression))(input)
}
pub fn l3_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = tag("^")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l4(input)?;
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
  let (input, args) = l2(input)?;
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

pub fn equality_math(input: &str) -> IResult<&str, Node> {
  let (input, side1) = alt((boolean, math_expression))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("==")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, side2) = alt((boolean, math_expression))(input)?;
  Ok((input, Node::TestEquality{children: vec![side1,side2]}))
}

pub fn condition(input: &str) -> IResult<&str, Node> {
  let (input, c1) = alt((parenthetical_condition, equality_math, boolean, function_call))(input)?;
  let (input, c2) = many0(condition_end)(input)?;
  Ok((input, Node::Condition{conditions: vec![c1]})) // my runtime function only considers the first condition.
  // I tried more but got really stuck. It's complicated!
}

pub fn condition_end(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("&&"),tag("&"),tag("||"),tag("|")))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, c2) = condition(input)?;
  Ok((input, Node::ConditionExpression{name: String::from(op), children: vec![c2]}))
}

pub fn if_stmt(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(alt((tag(" "),tag("\t"), tag("\n"))))(input)?;
  let (input, _) = tag("if ")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, result) = condition(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(alt((tag(" "),tag("\t"),tag("\n"))))(input)?;
  let (input, e) = many1(alt((statement, expression)))(input)?; // change this to be many statement/expression
  let (input, _) = many0(alt((tag(" "),tag("\t"),tag("\n"))))(input)?;
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag(" "),tag("\t"),tag("\n"))))(input)?;
  let mut c = vec![];
  //let mut s = vec![];
  for i in e {
    match i {
      Node::Expression{children} => {
        c.append(&mut vec![children[0].clone()]);
      }
      Node::Statement{children} => {
        c.append(&mut vec![children[0].clone()]);
      }
      _ => {
        panic!("Why isn't this an expression or statement");
      }
    }
  }
  
  match result {
    Node::Condition{conditions} => {
        Ok((input, Node::If{condition: conditions, children: c}))
    }
    Node::ConditionExpression{name, children} => {
      // create vector with all children later
        Ok((input, Node::If{condition: vec![Node::ConditionExpression{name:name, children: children}], children: c}))
    }
    _ => {
      panic!("Should be some conditions");
    }
  }
  
    
}



pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((boolean, if_stmt, math_expression, function_call, number, string, identifier))(input)?;
  Ok((input, Node::Expression{ children: vec![result]}))   
}

pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(alt((tag(" "),tag("\t"), tag("\n"), tag(" "))))(input)?;
  let (input, result) = alt((variable_define, function_return))(input)?;
  let (input, _) = tag(";")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = many0(tag("\n"))(input)?;
  Ok((input, Node::Statement{ children: vec![result]}))   
}
pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("return ")(input)?;
  let (input, return_value) = alt((function_call, expression, identifier))(input)?;
  Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
}
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
  let (input, arg) = expression(input)?;
  let (input, mut others) = many0(other_arg)(input)?;
  let mut args = vec![arg];
  args.append(&mut others);
  Ok((input, Node::FunctionArguments{children: args}))
}
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag(",")(input)?;
  expression(input)
}
pub fn function_definition(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("fn ")(input)?;
  let (input, function_name) = identifier(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, mut args) = many0(arguments)(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(tag("\n"))(input)?;
  let (input, mut statements) = many1(alt((statement, if_stmt)))(input)?;
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(alt((tag("\n"),tag(" "))))(input)?;
  let mut children = vec![function_name];
  children.append(&mut args);
  children.append(&mut statements);
  Ok((input, Node::FunctionDefine{ children: children }))   
}
// Define a program. You will change this, this is just here for example.
// You'll probably want to modify this by changing it to be that a program
// is defined as at least one function definition, but maybe more. Start
// by looking up the many1() combinator and that should get you started.
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = many1(alt((function_definition, statement, expression)))(input)?;  // Now that we've defined a number and an identifier, we can compose them using more combinators. Here we use the "alt" combinator to propose a choice.
  Ok((input, Node::Program{ children: result}))       // Whether the result is an identifier or a number, we attach that to the program
}
