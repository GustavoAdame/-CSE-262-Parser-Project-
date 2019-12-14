/*
Passed 18 test
boolean, string, math_expression, number, identifier, function_call
*/
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
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
}

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
  let(input, _) = tag("(")(input)?;
  let (input, result) = separated_list(tag(","), expression)(input)?;
  let(input, _) = tag(")")(input)?;
  Ok((input, Node::FunctionArguments{ children: result}))
}

pub fn number(input: &str) -> IResult<&str, Node> {
  let (input, result) = digit1(input)?;                    
  let number = result.parse::<i32>().unwrap();              
  Ok((input, Node::Number{ value: number}))                 
}

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

/*TODO:
 * Figure out parenthetical_expression
 * Figure out function_definition
 */
/*********************************************************************************************** */
pub fn program(input: &str) -> IResult<&str, Node> {
  let (input, result) = many1(alt((function_definition, statement, expression)))(input)?;  
  Ok((input, Node::Program{children: result}))       
} 

pub fn expression(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((function_call, boolean, string, math_expression, number, identifier))(input)?;
  Ok((input, Node::Expression{children: vec![result]}))    
}
/************************************************************************************** */
pub fn math_expression(input: &str) -> IResult<&str, Node> {
    l1(input)
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

pub fn l1_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("+"),tag("-")))(input)?;
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
pub fn l2_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = alt((tag("*"),tag("/")))(input)?;
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
pub fn l3_infix(input: &str) -> IResult<&str, Node> {
  let (input, _) = many0(tag(" "))(input)?;
  let (input, op) = tag("^")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, args) = l4(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l4(input: &str) -> IResult<&str, Node> {
  alt((parenthetical_expression, number, identifier, function_call))(input)
}
// Math expressions with parens (1 * (2 + 3))
pub fn parenthetical_expression(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("(")(input)?;
  let (input, result) = math_expression(input)?;
  let (input, _) = tag("(")(input)?;
  Ok((input, result))
}

/*** Define a statement of the form *********************************************************************** */
pub fn statement(input: &str) -> IResult<&str, Node> {
  let (input, result) = alt((function_return, variable_define))(input)?;
  let (input, _) = tag(";")(input)?;
  Ok((input, Node::Statement{children: vec![result]}))   
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

pub fn function_return(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("return")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, stat) = expression(input)?; 
  let (input, _) = tag(";")(input)?;
  Ok((input, Node::FunctionReturn{ children: vec![stat] })) 
}
/****************************************************************************************** */

pub fn function_definition(input: &str) -> IResult<&str, Node> {
  let (input, _) = tag("fn")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, name) = identifier(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("(")(input)?;
  let (input, arg) = arguments(input)?;
  let (input, _) = tag(")")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("{")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, mut stats) = many1(alt((statement, expression)))(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let (input, _) = tag("}")(input)?;
  let (input, _) = many0(tag(" "))(input)?;
  let mut theVec = vec![name, arg];
  theVec.append(&mut stats);
  Ok((input, Node::FunctionDefine { children: theVec }))
}




