use crate::{ast::*, token::TokenType};

struct Interpreter;

impl Visitor<Object> for Interpreter {
    fn visit_expr(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Binary(expr) => self.visit_binary_expr(expr),
            Expr::Grouping(expr) => self.visit_grouping_expr(expr),
            Expr::Literal(expr) => self.visit_literal_expr(expr),
            Expr::Unary(expr) => self.visit_unary_expr(expr),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Object {
        let left = evaluate(expr.left.clone());
        let right = evaluate(expr.right.clone());

        match (left, right) {
            (Object::Number(left), Object::Number(right)) => match expr.operator.variant {
                TokenType::MINUS => Object::Number(left - right),
                TokenType::SLASH => Object::Number(left / right),
                TokenType::STAR => Object::Number(left * right),
                TokenType::PLUS => Object::Number(left + right),
                TokenType::GREATER => Object::Boolean(left > right),
                TokenType::GREATER_EQUAL => Object::Boolean(left >= right),
                TokenType::LESS => Object::Boolean(left < right),
                TokenType::LESS_EQUAL => Object::Boolean(left <= right),
                TokenType::BANG_EQUAL => Object::Boolean(left != right),
                TokenType::EQUAL_EQUAL => Object::Boolean(left == right),
                _ => Object::Nil,
            },
            (Object::String(left), Object::String(right)) => match expr.operator.variant {
                TokenType::PLUS => Object::String(left + &right),
                TokenType::BANG_EQUAL => Object::Boolean(left != right),
                TokenType::EQUAL_EQUAL => Object::Boolean(left == right),
                _ => Object::Nil,
            },
            (Object::Boolean(left), Object::Boolean(right)) => match expr.operator.variant {
                TokenType::BANG_EQUAL => Object::Boolean(left != right),
                TokenType::EQUAL_EQUAL => Object::Boolean(left == right),
                _ => Object::Nil,
            },
            (Object::Nil, Object::Nil) => match expr.operator.variant {
                TokenType::BANG_EQUAL => Object::Boolean(false),
                TokenType::EQUAL_EQUAL => Object::Boolean(true),
                _ => Object::Nil,
            },
            _ => Object::Nil,
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Object {
        evaluate(expr.expression.clone())
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Object {
        expr.value.clone()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Object {
        let right = evaluate(expr.right.clone());

        match expr.operator.variant {
            TokenType::MINUS => {
                if let Object::Number(right) = right {
                    Object::Number(-right)
                } else {
                    Object::Nil
                }
            }
            TokenType::BANG => Object::Boolean(!is_truthy(right)),
            _ => Object::Nil,
        }
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Nil => false,
        Object::Boolean(value) => value,
        Object::String(value) => !value.is_empty(),
        Object::Number(value) => value != 0.0,
        Object::Identifier(_) => true,
    }
}

pub fn evaluate(expression: Expr) -> Object {
    Interpreter {}.visit_expr(&expression)
}
