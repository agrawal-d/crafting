use crate::ast::*;

/// Pretty prints the AST.
struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut retval = String::new();
        retval.push('(');
        retval.push_str(name);
        for expr in exprs {
            retval.push(' ');
            retval.push_str(&self.visit_expr(expr));
        }

        retval.push(')');

        retval
    }
}

/// Returns a string representation of the given expression.
pub fn print_expr(expr: Expr) -> String {
    AstPrinter {}.visit_expr(&expr)
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(expr) => self.visit_binary_expr(expr),
            Expr::Grouping(expr) => self.visit_grouping_expr(expr),
            Expr::Literal(expr) => self.visit_literal_expr(expr),
            Expr::Unary(expr) => self.visit_unary_expr(expr),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> String {
        self.parenthesize("group", vec![&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> String {
        match &expr.value {
            Object::Nil => "nil".to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Number(value) => value.to_string(),
            Object::String(value) => value.to_string(),
            Object::Identifier(value) => value.to_string(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right])
    }
}
