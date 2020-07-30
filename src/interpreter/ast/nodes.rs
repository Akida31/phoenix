use std::fmt::{Display, Formatter, Result};

use crate::interpreter::{Token, token::Sign, Position};
use crate::interpreter::token::types::Type;

#[derive(Clone)]
pub struct Node {
    ty: NodeType,
    pos: Position,
}

impl Node {
    pub fn new(ty: NodeType, pos: Position) -> Self {
        Self {
            ty,
            pos
        }
    }
    pub fn get_type(&self) -> NodeType {
        self.ty.clone()
    }

    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.ty)
    }
}


#[derive(Clone)]
pub enum NodeType {
    Node(Type),
    Operation(OperationType),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Node(node) => write!(f, "{}", node.to_string()),
            Self::Operation(op) => write!(f, "({})", op.to_string()),
        }
    }
}


#[derive(Clone)]
pub enum OperationType {
    BinaryOperationNode(Box<BinaryOperationNode>),
    UnaryOperationNode(Box<UnaryOperationNode>),
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
}

impl Display for BinaryOperationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    operator: Sign,
    node: Node,
}

impl UnaryOperationNode {
    pub fn new(operator: Sign, node: Node) -> Self {
        Self { operator, node }
    }

    pub fn get_node(&self) -> Node {
        self.node.clone()
    }
}

impl Display for UnaryOperationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}{}", self.operator, self.node)
    }
}
