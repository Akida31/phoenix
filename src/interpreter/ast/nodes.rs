use std::fmt::{self, Display, Formatter};

use crate::interpreter::token::ident::Ident;
use crate::interpreter::token::types::Type;
use crate::interpreter::{Position, Token};

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
    IfNode(IfNode),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(node) => write!(f, "{}", node),
            Self::Operation(op) => write!(f, "({})", op),
            Self::Assign(node) => write!(f, "{}", node),
            Self::Var(node) => write!(f, "{}", node),
            Self::IfNode(node) => write!(f, "{}", node),
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
pub struct IfNode {
    cases: Vec<(Node, Node)>,
    else_case: Box<Option<Node>>,
}

impl IfNode {
    pub fn new(cases: Vec<(Node, Node)>, else_case: Option<Node>) -> Self {
        Self {
            cases,
            else_case: Box::new(else_case),
        }
    }
    pub fn get_cases(&self) -> Vec<(Node, Node)> {
        self.cases.clone()
    }
    pub fn get_else_case(&self) -> Box<Option<Node>> {
        self.else_case.clone()
    }
}

impl Display for IfNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut cases = format!("if {} then {}", self.cases[0].1, self.cases[0].1);
        let mut it = self.cases.iter();
        it.next();
        for (case, expr) in it {
            cases = format!("{}\n   elif {} then {}", cases, case, expr);
        }
        if let Some(else_case) = *self.else_case.clone() {
            cases = format!("{}\n   else {}", cases, else_case);
        }
        write!(f, "{}", cases)
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
            Self::BinaryOperationNode(node) => write!(f, "{}", node),
            Self::UnaryOperationNode(node) => write!(f, "({})", node),
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
        write!(f, "{}{}{}", self.left, self.operation, self.right)
    }
}

#[derive(Clone, PartialEq)]
pub enum UnaryOperation {
    Plus,
    Minus,
    Not,
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Not => "!",
            }
        )
    }
}

impl UnaryOperation {
    pub fn from_token(t: Token) -> Option<Self> {
        match t {
            Token::Plus => Some(UnaryOperation::Plus),
            Token::Minus => Some(UnaryOperation::Minus),
            Token::Bang => Some(UnaryOperation::Not),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct UnaryOperationNode {
    operation: UnaryOperation,
    node: Node,
}

impl UnaryOperationNode {
    pub fn get_node(&self) -> Node {
        self.node.clone()
    }

    pub fn get_operation(&self) -> UnaryOperation {
        self.operation.clone()
    }

    pub fn from_token(t: Token, node: Node) -> Option<Self> {
        Some(Self {
            operation: UnaryOperation::from_token(t)?,
            node,
        })
    }
}

impl Display for UnaryOperationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operation, self.node)
    }
}
