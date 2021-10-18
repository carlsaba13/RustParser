extern crate nom;
extern crate cse262_project;

use cse262_project::{program, run, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  /*let (unparsed, ast) = program(r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#)?;
  println!("AST {:?}", ast);
  let result = run(&ast);
  println!("RESULT {:?}", result);*/
  Ok(())
}

