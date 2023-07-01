use craft::ast::*;
use craft::ast_printer::print_expr;
use craft::token::*;

fn main() {
    let left = Expr::Unary(Box::new(Unary::new(
        Token::new(TokenType::MINUS, "-".to_string(), 1),
        Expr::Literal(Box::new(Literal::new(Object::Number(123.0)))),
    )));

    let right = Expr::Grouping(Box::new(Grouping::new(Expr::Literal(Box::new(
        Literal::new(Object::Number(45.67)),
    )))));

    let expr = Expr::Binary(Box::new(Binary::new(
        left,
        Token::new(TokenType::STAR, "*".to_string(), 1),
        right,
    )));

    println!("{}", print_expr(expr));
}
