use crate::interpreter::ast::nodes::{
    Assignment, BinaryOperationNode, ForNode, IfNode, Node, NodeType, OperationType,
    UnaryOperation, UnaryOperationNode, WhileNode,
};
use crate::interpreter::token::ident::Ident;
use crate::interpreter::token::types::{Cmp, CmpResult, Integer};
use crate::interpreter::token::Token;
use crate::interpreter::{Context, Error, ErrorKind, Position, Type};

pub fn visit(node: Node, context: Context) -> Result<(Type, Context), Error> {
    let position = node.get_pos();
    match node.get_type() {
        NodeType::Node(ty) => Ok((ty, context)),
        NodeType::Operation(op) => match op {
            OperationType::BinaryOperationNode(op) => visit_binary_operation(*op, context),
            OperationType::UnaryOperationNode(op) => visit_unary_operation(*op, position, context),
        },
        NodeType::Var(id) => visit_var(id, context),
        NodeType::Assign(a) => visit_assignment(a, context),
        NodeType::IfNode(node) => visit_if_node(node, context),
        NodeType::ForNode(node) => visit_for_node(node, context),
        NodeType::WhileNode(node) => visit_while_node(node, context),
    }
}

fn visit_assignment(node: Assignment, context: Context) -> Result<(Type, Context), Error> {
    let value = visit(*node.get_expr(), context)?;
    let mut context = value.1;
    context.stack.set(node.get_name(), value.0.clone());
    Ok((value.0, context))
}

fn visit_var(node: Ident, context: Context) -> Result<(Type, Context), Error> {
    let value = context.stack.get(&node);
    match value {
        Some(val) => Ok((val, context)),
        None => Err(Error::new(
            ErrorKind::NameError,
            &*format!("{} is not defined", node.get()),
            Some(context.get_position()),
        )),
    }
}

fn visit_if_node(node: IfNode, context: Context) -> Result<(Type, Context), Error> {
    for (condition, expr) in node.get_cases().into_iter() {
        let condition_value = visit(condition, context.clone())?;
        if condition_value.0.as_conversion().__bool__()? {
            return visit(expr, context);
        }
    }
    if let Some(else_case) = *node.get_else_case() {
        visit(else_case, context)
    } else {
        Ok((Type::none(), context))
    }
}

fn visit_for_node(node: ForNode, context: Context) -> Result<(Type, Context), Error> {
    // TODO user can define step
    let step = 1;
    let (var_name, start, end, body) = node.get_all();
    let (start, context) = visit(start, context)?;
    let (end, mut context) = visit(end, context)?;
    context.stack.set(
        var_name.clone(),
        start.as_operators().sub(Type::Integer(Integer::new(1)))?,
    );
    let mut ty = Type::none();
    while end.cmp(
        context
            .stack
            .get(&var_name)
            .unwrap()
            .as_operators()
            .add(Type::Integer(Integer::new(step)))?,
    )? == CmpResult::Greater
    {
        context.stack.set(
            var_name.clone(),
            context
                .stack
                .get(&var_name)
                .unwrap()
                .as_operators()
                .add(Type::Integer(Integer::new(step)))?,
        );
        let visit_res = visit(body.clone(), context)?;
        ty = visit_res.0;
        context = visit_res.1;
    }
    Ok((ty, context))
}

fn visit_while_node(node: WhileNode, context: Context) -> Result<(Type, Context), Error> {
    let (cond, body) = node.get_all();
    let mut context = context;
    let mut ty = Type::none();
    loop {
        let res = visit(cond.clone(), context)?;
        context = res.1;
        if !res.0.as_conversion().__bool__()? {
            break Ok((ty, context));
        }
        let res = visit(body.clone(), context)?;
        ty = res.0;
        context = res.1;
    }
}

// TODO improve position marking
fn visit_binary_operation(
    node: BinaryOperationNode,
    context: Context,
) -> Result<(Type, Context), Error> {
    let (left_ty, left_ctx) = visit(node.get_left(), context.clone())?;
    let (right_ty, right_ctx) = visit(node.get_right(), context)?;
    let ctx = left_ctx.combine(right_ctx);
    let full = match node.get_operation() {
        Token::Plus => left_ty.as_operators().add(right_ty),
        Token::Minus => left_ty.as_operators().sub(right_ty),
        Token::Star => left_ty.as_operators().mul(right_ty),
        Token::Slash => left_ty.as_operators().div(right_ty),
        Token::DoubleEqual => left_ty.as_operators().eq(right_ty),
        Token::NonEqual => left_ty.as_operators().neq(right_ty),
        Token::LessThan => left_ty.as_operators().lt(right_ty),
        Token::GreaterThan => left_ty.as_operators().gt(right_ty),
        Token::LessThanEq => left_ty.as_operators().lte(right_ty),
        Token::GreaterThanEq => left_ty.as_operators().gte(right_ty),
        Token::DoubleAnd => left_ty.as_operators().and(right_ty),
        Token::DoubleOr => left_ty.as_operators().or(right_ty),
        t => Err(Error::new(
            ErrorKind::Undefined,
            &*format!("can't operate on token {}", t),
            Some(ctx.get_position()),
        )),
    };
    // fill position of full
    match full {
        Ok(f) => Ok((f, ctx)),
        Err(e) => Err(e.with_context(ctx)),
    }
}

fn visit_unary_operation(
    node: UnaryOperationNode,
    _pos: Position,
    context: Context,
) -> Result<(Type, Context), Error> {
    let (ty, ctx) = visit(node.get_node(), context.clone())?;
    let new_pos = Position::new(
        ctx.pos.index - 1,
        ctx.pos.filename,
        ctx.pos.line,
        ctx.pos.column,
        ctx.pos.len + 1,
    );
    let new_ctx = Context::new(new_pos, context.stack.clone(), Some(context));
    Ok((
        match node.get_operation() {
            UnaryOperation::Plus => ty,
            UnaryOperation::Minus => ty.as_operators().neg()?,
            UnaryOperation::Not => ty.as_operators().not()?,
        },
        new_ctx,
    ))
}
