use crate::token::Token;

/// Generates a AST node struct with the given name and fields.
/// Example:
/// ```
/// generate_node!(Node, field1: Type1, field2: Type2);
/// ```
macro_rules! generate_node {
    ($name:ident, $($field:ident : $type:ty),*) => {

        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

#[derive(Debug)]
pub enum Object {
    Number(f64),
    String(String),
    Identifier(String),
    Boolean(bool),
    Nil,
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
}

generate_node!(Binary, left: Expr, operator: Token, right: Expr);
generate_node!(Grouping, expression: Expr);
generate_node!(Literal, value: Object);
generate_node!(Unary, operator: Token, right: Expr);
