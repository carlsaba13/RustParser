extern crate nom;
extern crate cse262_project;

#[allow(unused_imports)]
use cse262_project::{program, run, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  /*let (unparsed, ast) = program(r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#)?;
  println!("AST {:?}", ast);
  let result = run(&ast);
  println!("RESULT {:?}", result);*/
  if !1+2==2 || !!false  {
    let x = 5;
    println!("Here");
  } else {
    println!("Not here");
  }
  Ok(())
}

