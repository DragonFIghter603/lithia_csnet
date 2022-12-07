use std::collections::HashMap;
use crate::source::{OnParseErr, ParseError, ParseET, Span};
use crate::tokens::tokens::Literal;

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Literal(AstLiteral),
    Variable(Ident),
    FuncCall(Item, Vec<Expression>),
    BinaryOp(Operator, Box<Expression>, Box<Expression>),
    UnaryOp(Operator, Box<Expression>)
}

#[derive(Debug, Clone)]
pub(crate) struct Operator(pub(crate) Op, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) enum Op {
    Add,
    Sub,
    Mul,
    Div,
    LShift,
    RShift,
}

impl Op {
    pub(crate) fn from_chars(chars: Vec<char>, loc: Option<Span>) -> Result<Op, ParseError>{
        Ok(match chars.iter().collect::<String>().as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            "<<" => Op::LShift,
            ">>" => Op::RShift,
            op => {
                let et = ParseET::ParsingError(format!("Operator '{op}' not recognized"));
                return Err(if let Some(span) = loc {
                    et.at(span)
                } else {
                    et.error()
                }).e_when(String::from("parsing operator"))
            }
        })
    }
}



#[derive(Debug, Clone)]
pub(crate) struct Expression(pub(crate) Expr, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) struct AstLiteral(pub(crate) Literal, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) struct Ident(pub(crate) String, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) struct Item(pub(crate) Vec<Ident>, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) struct FullType(pub(crate) TypeT, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) enum TypeT {
    Single(Type),
    Tuple(Vec<FullType>)
}

impl TypeT {
    pub(crate) fn empty() -> Self{
        TypeT::Tuple(vec![])
    }
    pub(crate) fn is_empty(&self) -> bool {
        if let TypeT::Tuple(ty) = self {
            ty.len() == 0
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Type {
    pub(crate) generics: Vec<FullType>,
    pub(crate) base_type: Item,
    pub(crate) loc: Span
}

#[derive(Debug, Clone)]
pub(crate) struct Func {
    pub(crate) name: Ident,
    pub(crate) args: Vec<(Ident, FullType)>,
    pub(crate) ret: FullType,
    pub(crate) body: Block,
    pub(crate) loc: Span
}

#[derive(Debug, Clone)]
pub(crate) enum Stmt {
    Expression(Expression),
    VarCreate(Item, Self::mutable, Option<FullType>, Expression),
    VarAssign(Item, Option<Operator>, Expression)
}

impl Stmt {
    type mutable = bool;
}

#[derive(Debug, Clone)]
pub(crate) struct Statement(pub(crate) Stmt, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) struct Block(pub(crate) Vec<Statement>, pub(crate) Span);

#[derive(Debug, Clone)]
pub(crate) struct Module{
    pub(crate) name: Ident,
    pub(crate) sub_modules: Map<Module>,
    pub(crate) functions: Map<Func>,
    pub(crate) loc: Span
}

pub(crate) type Map<T> = HashMap<String, T>;