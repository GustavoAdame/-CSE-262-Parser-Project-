// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/5.0.1/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
  IResult,
  separated_list,
  branch::alt,
  sequence::delimited,
  combinator::opt,
  multi::{many1, many0, separated_list},
  bytes::complete::{tag},
  character::complete::{alphanumeric1, digit1, space1},
};


#[derive(Debug, Clone)]
pub enum Node {
  //Nodes with children
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  VariableDefine { children: Vec<Node> },

  //Nodes with name and children 
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },

  //Nodes that is value
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
}

/***********
 * TODO:
 * * Figure out arguments function
 * * Function_call
 * * Figure out function definition and statement
 * * function args sperlated list (",",exp )
 ******************************************************/


/*** Working On ****************************************************************************** */
pub fn function_call(input: &str) -> IResult<&str, Node> {
  let (input, result) = identifier(input)?;
  let function_name: String = match result {
    Node::Identifier{value} => value.clone(),
    _ => "".to_string(),
  };
  let (input, args) = arguments(input)?;
  Ok((input, Node::FunctionCall{name: function_name, children: vec![args]}))
}

pub fn arguments(input: &str) -> IResult<&str, Node> {
  let exp = string(input)?;
  //let (input, result) = separated_list!(",", exp)(input)?;
  Ok((input, Node::FunctionArguments{ children: vec![result]}))
}
/*********************************************************************************************** */
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((expression, statement))(input)?;  
  Ok((input, Node::Program{children: vec![result]}))       
}

pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((number, function_call, string, boolean, identifier))(input)?;
  Ok((input, Node::Expression{children: vec![result]}))    
}

// pub fn math_expression(input: &str) -> IResult<&str, Node> {
//   l1(input)
// }

pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, result) = digit1(input)?;                    
  let number = result.parse::<i32>().unwrap();              
  Ok((input, Node::Number{ value: number}))                 
}

// pub fn function_call(input: &str) -> IResult<&str, Node> {
//
// }

pub fn string(input: &str) -> IResult<&str, Node> {  
  let (input, result) = delimited(tag("\""), many1(alt((alphanumeric1, space1))), tag("\""))(input)?;
  let mut result_string = String::new();
  for x in result {
    result_string.push_str(&x.to_string())
}
  Ok((input, Node::String{ value: result_string}))
}

pub fn boolean(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((tag("true"),tag("false")))(input)?;
  let bool_value = if result == "true" {true} else {false};
  Ok((input, Node::Bool{ value: bool_value})) 
}

pub fn identifier(input: &str) -> IResult<&str, Node> {
  let (input, result) = alphanumeric1(input)?;              
  Ok((input, Node::Identifier{ value: result.to_string()})) 
}

pub fn comment(input: &str) -> IResult<&str, Node> {
  let (input, result) = identifier(input)?;             
  Ok((input, Node::Identifier{value: "".to_string()}))
}
/************************************************************************************** */

pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, result) = expression(input)?;
  Ok((input, Node::FunctionReturn{children: vec![result]}))    
}

// Math expressions with parens (1 * (2 + 3))
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
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



pub fn statement(input: &str) -> IResult<&str, Node> {
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

// Like the first argument but with a comma in front
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

pub fn function_definition(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}




