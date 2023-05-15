use crate::{lexer, parser, ast};

#[test]
fn test_let_statement() {
    let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ";
    let mut lex = lexer::Lexer::new(input);
    let mut p = parser::Parser::new(lex);
    let program = p.parse_program();
    // check_parse_errors(&p);
    let statements = program.as_ref().unwrap().statements();
    assert_eq!(program.as_ref().unwrap().statements().len(), 3);
    let expected = vec![
        "x",
        "y",
        "foobar",
    ];
    for (i, tt) in expected.iter().enumerate() {
        let stmt = &statements[i];
        assert_eq!(stmt.token_literal(), "let");
        assert_eq!(stmt.token_literal(), *tt);
        let let_stmt = stmt.as_any().downcast_ref::<ast::LetStatement>().unwrap();
        assert_eq!(let_stmt.name().value(), *tt);
    }
}