use crate::token::Token;

/// Generates a AST node struct with the given name and fields.
/// Example:
/// ```
/// generate_node!(Node, field1: Type1, field2: Type2);
/// ```
macro_rules! generate_node {
    ($name:ident, $($field:ident : $type:ty),*) => {

        #[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

#[derive(Debug, Clone)]
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

pub trait StmtVisitor<T> {
    fn visit_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
}

generate_node!(Binary, left: Expr, operator: Token, right: Expr);
generate_node!(Grouping, expression: Expr);
generate_node!(Literal, value: Object);
generate_node!(Unary, operator: Token, right: Expr);

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Box<Expression>),
    Print(Box<Print>),
    Empty,
}

generate_node!(Expression, expr: Expr);
generate_node!(Print, expr: Expr);
