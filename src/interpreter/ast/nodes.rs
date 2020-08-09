use std::fmt::{self, Display, Formatter};

use crate::interpreter::token::ident::Ident;
use crate::interpreter::token::types::Type;
use crate::interpreter::{token::Sign, Position, Token};

#[derive(Clone)]
pub struct Node {
    ty: NodeType,
    pos: Position,
}

impl Node {
    pub fn new(ty: NodeType, pos: Position) -> Self {
        Self { ty, pos }
    }
    pub fn get_type(&self) -> NodeType {
        self.ty.clone()
    }

    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)
    }
}

#[derive(Clone)]
pub enum NodeType {
    Node(Type),
    Operation(OperationType),
    Assign(Assignment),
    Var(Ident),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(node) => write!(f, "{}", node),
            Self::Operation(op) => write!(f, "({})", op),
            Self::Assign(node) => write!(f, "{}", node),
            Self::Var(node) => write!(f, "{}", node),
        }
    }
}

#[derive(Clone)]
pub struct Assignment {
    name: Ident,
    expr: Box<Node>,
}

impl Assignment {
    pub fn new(name: Ident, expr: Node) -> Self {
        Self {
            name,
            expr: Box::new(expr),
        }
    }

    pub fn get_name(&self) -> Ident {
        self.name.clone()
    }
    pub fn get_expr(&self) -> Box<Node> {
        self.expr.clone()
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.expr)
    }
}

#[derive(Clone)]
pub enum OperationType {
    BinaryOperationNode(Box<BinaryOperationNode>),
    UnaryOperationNode(Box<UnaryOperationNode>),
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinaryOperationNode(node) => write!(f, "{}", node.to_string()),
            Self::UnaryOperationNode(node) => write!(f, "({})", node.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct BinaryOperationNode {
    left: Node,
    operation: Token,
    right: Node,
}

impl BinaryOperationNode {
    pub fn new(left: Node, operation: Token, right: Node) -> Self {
        Self {
            left,
            operation,
            right,
        }
    }

    pub fn get_left(&self) -> Node {
        self.left.clone()
    }

    pub fn get_right(&self) -> Node {
        self.right.clone()
    }

    pub fn get_operation(&self) -> Token {
        self.operation.clone()
    }
}

impl Display for BinaryOperationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.left.to_string(),
            self.operation,
            self.right.to_string()
        )
    }
}

#[derive(Clone)]
pub struct UnaryOperationNode {
    operation: Sign,
    node: Node,
}

impl UnaryOperationNode {
    pub fn new(operation: Sign, node: Node) -> Self {
        Self { operation, node }
    }

    pub fn get_node(&self) -> Node {
        self.node.clone()
    }

    pub fn get_operation(&self) -> Sign {
        self.operation.clone()
    }
}

impl Display for UnaryOperationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operation, self.node)
    }
}
