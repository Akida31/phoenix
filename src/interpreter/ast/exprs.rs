use crate::interpreter::ast::nodes::{
    Assignment, ForNode, IfNode, Node, NodeType, OperationType, UnaryOperationNode, WhileNode,
};
use crate::interpreter::ast::Parser;
use crate::interpreter::token::keyword::Keyword;
use crate::interpreter::token::Token;
use crate::interpreter::{Error, ErrorKind, Position};

pub fn expr(parser: &mut Parser) -> Result<Node, Error> {
    let current_token = parser.current_token();
    if current_token.is_some() && current_token.unwrap().0 == Token::Keyword(Keyword::Let) {
        parser.advance();
        if let Some(c) = parser.current_token() {
            let pos = c.1;
            if let Token::Ident(i) = c.0 {
                let name = i;
                parser.advance();
                if let Some(c) = parser.current_token() {
                    if Token::Equal == c.0 {
                        parser.advance();
                        let expr = expr(parser)?;
                        return Ok(Node::new(
                            NodeType::Assign(Assignment::new(name, expr)),
                            pos,
                        ));
                    }
                }
                Err(Error::new(ErrorKind::SyntaxError, "expected =", Some(pos)))
            } else {
                Err(Error::new(
                    ErrorKind::SyntaxError,
                    "expected identifier",
                    Some(pos),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::SyntaxError,
                "expected expression",
                None,
            ))
        }
    } else {
        parser.binary_operation(&mut comp_expr, vec![Token::DoubleAnd, Token::DoubleOr])
    }
}

pub fn if_expr(parser: &mut Parser, position: Position) -> Result<Node, Error> {
    parser.advance();
    let condition = expr(parser)?;
    let expression = expr(parser)?;
    let mut cases = vec![(condition, expression)];
    while let Some((Token::Keyword(Keyword::Elif), _pos)) = parser.current_token() {
        parser.advance();
        let condition = expr(parser)?;
        let expression = expr(parser)?;
        cases.push((condition, expression));
    }
    let else_case = if let Some((Token::Keyword(Keyword::Else), _pos)) = parser.current_token() {
        parser.advance();
        Some(expr(parser)?)
    } else {
        None
    };
    let pos = if let Some((_, pos)) = parser.current_token() {
        position.combine(pos)
    } else {
        position
    };
    Ok(Node::new(
        NodeType::IfNode(IfNode::new(cases, else_case)),
        pos,
    ))
}

pub fn while_expr(parser: &mut Parser, position: Position) -> Result<Node, Error> {
    parser.advance();
    let cond = expr(parser)?;
    if let Some((Token::Keyword(Keyword::Then), _)) = parser.current_token() {
        parser.advance();
        let body = expr(parser)?;
        let pos = position.combine(body.get_pos());
        Ok(Node::new(
            NodeType::WhileNode(WhileNode::new(cond, body)),
            pos,
        ))
    } else {
        Err(Error::new(
            ErrorKind::SyntaxError,
            "expected 'then'",
            Some(position),
        ))
    }
}

pub fn for_expr(parser: &mut Parser, position: Position) -> Result<Node, Error> {
    // TODO improve position of Errors
    parser.advance();
    if let Some((Token::Ident(var_name), _)) = parser.current_token() {
        parser.advance();
        if let Some((Token::Keyword(Keyword::In), _)) = parser.current_token() {
            parser.advance();
            let start = expr(parser)?;
            if let Some((Token::Keyword(Keyword::To), _)) = parser.current_token() {
                parser.advance();
                let end = expr(parser)?;
                if let Some((Token::Keyword(Keyword::Then), _)) = parser.current_token() {
                    parser.advance();
                    let body = expr(parser)?;
                    let pos = position.combine(end.get_pos());
                    Ok(Node::new(
                        NodeType::ForNode(ForNode::new(var_name, start, end, body)),
                        pos,
                    ))
                } else {
                    Err(Error::new(
                        ErrorKind::SyntaxError,
                        "expected 'then'",
                        Some(position),
                    ))
                }
            } else {
                Err(Error::new(
                    ErrorKind::SyntaxError,
                    "expected 'to'",
                    Some(position),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::SyntaxError,
                "expected 'in'",
                Some(position),
            ))
        }
    } else {
        Err(Error::new(
            ErrorKind::SyntaxError,
            "expected ident",
            Some(position),
        ))
    }
}

fn comp_expr(parser: &mut Parser) -> Result<Node, Error> {
    if let Some((Token::Bang, pos)) = parser.current_token() {
        parser.advance();
        let node = comp_expr(parser)?;
        Ok(Node::new(
            NodeType::Operation(OperationType::UnaryOperationNode(Box::new(
                UnaryOperationNode::from_token(Token::Bang, node).unwrap(),
            ))),
            pos,
        ))
    } else {
        parser.binary_operation(
            &mut arith_expr,
            vec![
                Token::DoubleEqual,
                Token::NonEqual,
                Token::LessThan,
                Token::GreaterThan,
                Token::LessThanEq,
                Token::GreaterThanEq,
            ],
        )
    }
}

fn arith_expr(parser: &mut Parser) -> Result<Node, Error> {
    parser.binary_operation(&mut super::term, vec![Token::Plus, Token::Minus])
}
