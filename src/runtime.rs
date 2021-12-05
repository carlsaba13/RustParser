use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
  Ignore()
}


struct Runtime {
  functions: HashMap<String, Vec<Node>>,
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    println!("Node = {:?}", node);
    match node {
      Node::Program{children} => {
        for n in children {
          match n {
            Node::FunctionDefine{..} => {
              self.run(n);
            },
            Node::Expression{..} => {
              self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![n.clone()]}]);
            },
            Node::Statement{..} => {
              self.functions.insert("main".to_string(), vec![n.clone()]);
            }
            _ => (),
          }
        }
        Ok(Value::Bool(true))
      },
      Node::MathExpression{name, children} => {
        match (self.run(&children[0]), self.run(&children[1])) {
          (Ok(Value::Number(lhs)), Ok(Value::Number(rhs))) => {
            match name.as_ref() {
              "+" => Ok(Value::Number(lhs + rhs)),
              "-" => Ok(Value::Number(lhs - rhs)),
              "*" => Ok(Value::Number(lhs * rhs)),
              "/" => Ok(Value::Number(lhs / rhs)),
              "^" => {
                let mut result = 1;
                for i in 0..rhs {
                  result = result * lhs;
                }
                Ok(Value::Number(result))
              },
              _ => Err("Undefined operator"),
            }
          }
          _ => Err("Cannot do math on String or Bool"),
        }
      },
      Node::FunctionCall{name, children} => {
        let in_args = if children.len() > 0 {
          match &children[0] {
            Node::FunctionArguments{children} => {
              children
            },
            _ => children,
          }
        } else {
          children
        };
        let mut new_frame = HashMap::new();
        let mut result: Result<Value, &'static str> = Err("Undefined function");
        let rt = self as *mut Runtime;
        match self.functions.get(name) {
          Some(statements) => {
            {
              match statements[0].clone() { 
                Node::FunctionArguments{children} => {
                  
                  for (ix, arg) in children.iter().enumerate() {
                    unsafe {
                      let result = (*rt).run(&in_args[ix])?;
                      match arg {
                        Node::Expression{children} => {
                          match &children[0] {
                            Node::Identifier{value} => {
                              new_frame.insert(value.clone(),result);
                            },
                            _ => (),
                          }
                        }
                        _ => (),
                      }
                    }
                  }
                }
                _ => (),
              }     
            }  
            self.stack.push(new_frame);
            for n in statements.clone() {
              result = self.run(&n);
            }
            self.stack.pop();
          },
          None => (),
        };
        result
      },
      Node::FunctionDefine{children} => {
        let (head, tail) = children.split_at(1);
        match &head[0] {
          Node::Identifier{value} => {
            self.functions.insert(value.to_string(), tail.to_vec());
          },
          _ => (),
        }
        Ok(Value::Bool(true))
      },
      Node::FunctionReturn{children} => {
        self.run(&children[0])
      },
      Node::Identifier{value} => {
        let last = self.stack.len() - 1;
        match self.stack[last].get(value) {
          Some(id_value) => Ok(id_value.clone()),
          None => Err("Undefined variable"),
        }
      },
      Node::Statement{children} => {
        match children[0] {
          Node::VariableDefine{..} |
          Node::FunctionReturn{..} => {
            self.run(&children[0])
          },
          _ => Err("Unknown Statement"),
        }
      },
      Node::VariableDefine{children} => {
        // Variable name
        let name: String = match &children[0] {
          Node::Identifier{value} => value.clone(),
          _ => "".to_string(),
        };
        // Expression result
        let value = self.run(&children[1])?;
        let last = self.stack.len() - 1;
        self.stack[last].insert(name, value.clone());
        Ok(value)
      }
      Node::Expression{children} => {
        println!("Runtime expression");
        println!("{:?}", children[0]);
        match &children[0] {
          Node::If{condition, children} => {
            let r = self.run(&condition[0])?;
            println!("r = {:?}", r);
            match r {
              Value::Bool(val) => {
                if val {
                  self.run(&children[0])
                } else {
                  Ok(Value::Ignore())
                }
              }
              _ => {
                Err("Why isn't this a bool")
              }
            }
          }
          Node::MathExpression{..} |
          Node::Number{..} |
          Node::FunctionCall{..} |
          Node::String{..} |
          Node::Bool{..} |
          Node::Identifier{..} => {
            self.run(&children[0])
          },
          _ => Err("Unknown Expression"),
        }
      }
      Node::Number{value} => {
        Ok(Value::Number(*value))
      }
      Node::String{value} => {
        Ok(Value::String(value.clone()))
      }
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      }
      Node::If{condition, children} => {
        if condition.len() % 2 != 1 { // should be an odd amount of conditions
          // Ex 'true | false' = 3 conditions
          // Ex 'true' = 1 condition
          // Ex 1+1==2 && true = 3 conditions
          return Err("Invalid amount of conditions")
        }
        let r = self.run(&condition[0])?;
        println!("r = {:?}", r);
        match r {
          Value::Bool(val) => {
            if val {
              self.run(&children[0])
            } else {
              Ok(Value::Ignore())
            }
          }
          Value::Ignore() => {
            Ok(Value::Ignore())
          }
          _ => {
            Err("Why isn't this a bool")
          }
        }
      }
      Node::TestEquality{children} => {
        /*let s1 = self.run(&children[0])?;
        let s2 = self.run(&children[1])?;
        println!("Side 1 = {:?}", s1);
        println!("Side 2 = {:?}", s2);*/
        match (self.run(&children[0]), self.run(&children[1])) {
          (Ok(Value::Number(lhs)), Ok(Value::Number(rhs))) => {
            println!("Two numbers");
            Ok(Value::Bool(lhs == rhs))
          }
          (Ok(Value::Bool(lhs)), Ok(Value::Bool(rhs))) => {
            println!("Two bools");
            Ok(Value::Bool(lhs == rhs))
          }
          _ => {
            println!("lhs = {:?}", self.run(&children[0]));
            println!("rhs = {:?}", self.run(&children[1]));
            return Err("Incompatible types")
          }
        }
      }
      Node::ConditionExpression {
        
      }
      _ => {
        Err("Unhandled Node")
      },
    }
  }

}

pub fn run(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  runtime.run(node);
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}