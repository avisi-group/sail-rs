#![allow(missing_docs)]

//! JIB AST
//!
//! JIB abstract syntax tree corresponding to data structures in `jib.ml`,
//! which itself is generated from `jib.lem` and `jib.ott`.

use {
    crate::{
        jib_ast::visitor::{Visitor, Walkable},
        num::BigInt,
        sail_ast::{Identifier, KindIdentifier, Location},
    },
    common::intern::InternedString,
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Int, ToValue},
    serde::{Deserialize, Serialize},
    std::collections::LinkedList,
};

pub mod pretty_print;
pub mod visitor;

/// C type
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Type {
    Lint,
    Fint(Int),
    Constant(BigInt),
    Lbits,
    Sbits(Int),
    Fbits(Int),
    Unit,
    Bool,
    Bit,
    String,
    Real,
    Float(Int),
    RoundingMode,
    Tup(LinkedList<Self>),
    Enum(Identifier, LinkedList<Identifier>),
    Struct(Identifier, LinkedList<(Identifier, Self)>),
    Variant(Identifier, LinkedList<(Identifier, Self)>),
    Fvector(Int, Box<Self>),
    Vector(Box<Self>),
    List(Box<Self>),
    Ref(Box<Self>),
    Poly(KindIdentifier),
}

impl Walkable for Type {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Lint => (),
            Self::Fint(_) => (),
            Self::Constant(_) => (),
            Self::Lbits => (),
            Self::Sbits(_) => (),
            Self::Fbits(_) => (),
            Self::Unit => (),
            Self::Bool => (),
            Self::Bit => (),
            Self::String => (),
            Self::Real => (),
            Self::Float(_) => (),
            Self::RoundingMode => (),
            Self::Tup(types) => types.iter().for_each(|t| visitor.visit_type(t)),
            Self::Enum(_, _) => (),
            Self::Struct(_, fields) | Self::Variant(_, fields) => {
                fields.iter().for_each(|(_, typ)| {
                    visitor.visit_type(typ);
                });
            }
            Self::Fvector(_, typ) | Self::Vector(typ) | Self::List(typ) | Self::Ref(typ) => {
                visitor.visit_type(typ)
            }
            Self::Poly(_) => (),
        }
    }
}

/// Name
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Name {
    Name(Identifier, Int),
    Global(Identifier, Int),
    HaveException(Int),
    CurrentException(Int),
    ThrowLocation(Int),
    Return(Int),
}

impl Walkable for Name {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// Operation

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Op {
    Bnot,
    Bor,
    Band,
    ListHead,
    ListTail,
    ListIsEmpty,
    Eq,
    Neq,
    Ilt,
    Ilteq,
    Igt,
    Igteq,
    Iadd,
    Isub,
    Unsigned(Int),
    Signed(Int),
    Bvnot,
    Bvor,
    Bvand,
    Bvxor,
    Bvadd,
    Bvsub,
    Bvaccess,
    Concat,
    ZeroExtend(Int),
    SignExtend(Int),
    Slice(Int),
    Sslice(Int),
    SetSlice,
    Replicate(Int),
}

impl Walkable for Op {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// C value

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Value {
    Id(Name, Type),
    Lit(Vl, Type),
    Tuple(LinkedList<Self>, Type),
    Struct(LinkedList<(Identifier, Self)>, Type),
    CtorKind(Box<Self>, Identifier, LinkedList<Type>, Type),
    CtorUnwrap(Box<Self>, Identifier, LinkedList<Type>, Type),
    TupleMember(Box<Self>, Int, Int),
    Call(Op, LinkedList<Self>),
    Field(Box<Self>, Identifier),
}

impl Walkable for Value {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Id(name, typ) => {
                visitor.visit_name(name);
                visitor.visit_type(typ);
            }
            Self::Lit(vl, typ) => {
                visitor.visit_vl(vl);
                visitor.visit_type(typ);
            }
            Self::Tuple(values, typ) => {
                values.iter().for_each(|value| visitor.visit_value(value));
                visitor.visit_type(typ);
            }
            Self::Struct(fields, typ) => {
                fields.iter().for_each(|(_, value)| {
                    visitor.visit_value(value);
                });
                visitor.visit_type(typ);
            }
            Self::CtorKind(value, _, types, typ) => {
                visitor.visit_value(value);
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            Self::CtorUnwrap(value, _, types, typ) => {
                visitor.visit_value(value);
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            Self::TupleMember(value, _, _) => visitor.visit_value(value),
            Self::Call(op, values) => {
                visitor.visit_op(op);
                values.iter().for_each(|value| visitor.visit_value(value));
            }
            Self::Field(value, _) => {
                visitor.visit_value(value);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum BitU {
    B0,
    B1,
    BU,
}

/// Value2.vl

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Vl {
    Bits(LinkedList<BitU>),
    Bit(BitU),
    Bool(bool),
    Unit,
    Int(BigInt),
    String(InternedString),
    Real(InternedString),
    Enum(InternedString),
    Ref(InternedString),
    Undefined,
}

impl Walkable for Vl {
    fn walk<V: Visitor>(&self, _: &mut V) {
        // leaf node
    }
}

/// clexp?

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Expression {
    Id(Name, Type),
    Rmw(Name, Name, Type),
    Field(Box<Self>, Identifier),
    Addr(Box<Self>),
    Tuple(Box<Self>, Int),
    Void,
}

impl Walkable for Expression {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Id(name, typ) => {
                visitor.visit_name(name);
                visitor.visit_type(typ);
            }
            Self::Rmw(name0, name1, typ) => {
                visitor.visit_name(name0);
                visitor.visit_name(name1);
                visitor.visit_type(typ);
            }
            Self::Field(expression, _) => {
                visitor.visit_expression(expression);
            }
            Self::Addr(expression) => visitor.visit_expression(expression),
            Self::Tuple(expression, _) => visitor.visit_expression(expression),
            Self::Void => (),
        }
    }
}

type InstructionAnnotation = (Int, Location);

/// C type definition

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum TypeDefinition {
    Enum(Identifier, LinkedList<Identifier>),
    Struct(Identifier, LinkedList<(Identifier, Type)>),
    Variant(Identifier, LinkedList<(Identifier, Type)>),
}

impl Walkable for TypeDefinition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Enum(_, _) => (),
            Self::Struct(_, fields) | Self::Variant(_, fields) => {
                fields.iter().for_each(|(_, typ)| {
                    visitor.visit_type(typ);
                });
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum InstructionAux {
    Decl(Type, Name),
    Init(Type, Name, Value),
    Jump(Value, InternedString),
    Goto(InternedString),
    Label(InternedString),
    Funcall(
        Expression,
        bool,
        Identifier,
        LinkedList<Type>,
        LinkedList<Value>,
    ),
    Copy(Expression, Value),
    Clear(Type, Name),
    Undefined(Type),
    Exit(InternedString),
    End(Name),
    If(
        Value,
        LinkedList<Instruction>,
        LinkedList<Instruction>,
        Type,
    ),
    Block(LinkedList<Instruction>),
    TryBlock(LinkedList<Instruction>),
    Throw(Value),
    Comment(InternedString),
    Raw(InternedString),
    Return(Value),
    Reset(Type, Name),
    Reinit(Type, Name, Value),
}

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub struct Instruction {
    pub inner: InstructionAux,
    pub annot: InstructionAnnotation,
}

impl Walkable for Instruction {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match &self.inner {
            InstructionAux::Decl(typ, name) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
            }
            InstructionAux::Init(typ, name, value) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
                visitor.visit_value(value);
            }
            InstructionAux::Jump(value, _) => visitor.visit_value(value),
            InstructionAux::Goto(_) => {}
            InstructionAux::Label(_) => {}
            InstructionAux::Funcall(expression, _, _, parameter_types, parameters) => {
                visitor.visit_expression(expression);
                parameter_types
                    .iter()
                    .for_each(|typ| visitor.visit_type(typ));
                parameters
                    .iter()
                    .for_each(|value| visitor.visit_value(value));
            }
            InstructionAux::Copy(expression, value) => {
                visitor.visit_expression(expression);
                visitor.visit_value(value);
            }
            InstructionAux::Clear(typ, name) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
            }
            InstructionAux::Undefined(typ) => visitor.visit_type(typ),
            InstructionAux::Exit(_) => {}
            InstructionAux::End(name) => visitor.visit_name(name),
            InstructionAux::If(value, if_body, else_body, typ) => {
                visitor.visit_value(value);
                if_body.iter().for_each(|i| visitor.visit_instruction(i));
                else_body.iter().for_each(|i| visitor.visit_instruction(i));
                visitor.visit_type(typ);
            }
            InstructionAux::Block(instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            InstructionAux::TryBlock(instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            InstructionAux::Throw(value) => visitor.visit_value(value),
            InstructionAux::Comment(_) => {}
            InstructionAux::Raw(_) => {}
            InstructionAux::Return(value) => visitor.visit_value(value),
            InstructionAux::Reset(typ, name) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
            }
            InstructionAux::Reinit(typ, name, value) => {
                visitor.visit_type(typ);
                visitor.visit_name(name);
                visitor.visit_value(value);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum Definition {
    Register(Identifier, Type, LinkedList<Instruction>),
    Type(TypeDefinition),
    Let(Int, LinkedList<(Identifier, Type)>, LinkedList<Instruction>),
    Val(Identifier, Option<InternedString>, LinkedList<Type>, Type),
    Fundef(
        Identifier,
        Option<Identifier>,
        LinkedList<Identifier>,
        LinkedList<Instruction>,
    ),
    Startup(Identifier, LinkedList<Instruction>),
    Finish(Identifier, LinkedList<Instruction>),
    Pragma(InternedString, InternedString),
}

impl Walkable for Definition {
    fn walk<V: Visitor>(&self, visitor: &mut V) {
        match self {
            Self::Register(_, typ, instructions) => {
                visitor.visit_type(typ);
                instructions
                    .iter()
                    .for_each(|i| visitor.visit_instruction(i));
            }
            Self::Type(type_definition) => visitor.visit_type_definition(type_definition),
            Self::Let(_, types, instructions) => {
                types.iter().for_each(|(_, typ)| visitor.visit_type(typ));
                instructions
                    .iter()
                    .for_each(|i| visitor.visit_instruction(i));
            }
            Self::Val(_, _, types, typ) => {
                types.iter().for_each(|typ| visitor.visit_type(typ));
                visitor.visit_type(typ)
            }
            Self::Fundef(_, _, _, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            Self::Startup(_, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            Self::Finish(_, instructions) => instructions
                .iter()
                .for_each(|i| visitor.visit_instruction(i)),
            Self::Pragma(_, _) => (),
        }
    }
}
