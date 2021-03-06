extern crate cse262_project;
extern crate nom;

#[allow(unused_imports)]
use cse262_project::{program, Node, Value, run};
use nom::IResult;


macro_rules! test {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),nom::Err<(&'static str, nom::error::ErrorKind)>> {
      match program($test) {
        Ok((input, p)) => {
          assert_eq!(input, "");
          println!("p = {:?}", p);
          println!("run(p) = {:?}", run(&p));
          assert_eq!(run(&p), $expected);
          Ok(())
        },
        Err(e) => Err(e),
      }
    }
  )
}

test!(numeric, r#"123"#, Ok(Value::Number(123)));
test!(identifier, r#"x"#, Err("Undefined variable"));
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string())));
test!(bool_true, r#"true"#, Ok(Value::Bool(true)));
test!(bool_false, r#"false"#, Ok(Value::Bool(false)));
test!(function_call, r#"foo()"#, Err("Undefined function"));
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function"));
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function"));
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test!(math, r#"1 + 1"#, Ok(Value::Number(2)));
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2)));
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0)));
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8)));
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16)));
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22)));
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18)));
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function"));
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function"));
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6)));
test!(define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));
test!(binary, r#"0b1111011"#, Ok(Value::Number(123)));
test!(octal, r#"0o173"#, Ok(Value::Number(123)));
test!(multiple_conditions_if_stmts, r#"
if  !(true) && false  {
  if (true) {
    let x = 1;
  } else if (false || true) && (1+1)*2==2 {
    let y = 2;
  } else {
    let y = 4;
  }
} else if (1+1)==2 {
  let x = 3;
} else {
  if 1+1==3 {
    let y = 1;
  }
  let y = 1;
  let s = "This parser works pretty much how I expect";
  
}
"#, Err("Unknown Expression"));
/*test!(define_full_program_with_comments, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  // x = 5;
  return x * y;
}

fn bar(a) {
  // hi there!
  // I can do whatever I want here
  return a * 3;
}

fn main() {
  // please give me a good grade
  // I worked very hard
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));*/


