use noir_field::FieldElement;

use crate::parser::errors::ParserErrorKind;

use super::*;

pub struct CallParser;

impl CallParser {
    /// Parses call expressions of the form
    ///
    /// PATH (<EXPR> <EXPR> <EXPR> ... )
    ///
    /// Cursor Start : `(`
    ///
    /// Cursor End : `)`
    pub fn parse<F: FieldElement>(
        parser: &mut Parser<F>,
        func_path: Expression<F>,
    ) -> ParserExprKindResult<F> {
        assert_eq!(&parser.curr_token, &Token::LeftParen);

        //
        // Parse arguments in the call expression
        let arguments = parser.parse_comma_separated_argument_list(Token::RightParen)?;

        let func_name = match func_path.kind {
            ExpressionKind::Path(path) => path,
            _ => {
                return Err(ParserErrorKind::UnstructuredError {
                    message: "expected a path for the function name".to_string(),
                    span: func_path.span,
                })
            }
        };

        // The cursor position is inherited from the argument parsing
        // procedure which is `)`

        let call_expr = CallExpression {
            func_name,
            arguments,
        };

        Ok(ExpressionKind::Call(Box::new(call_expr)))
    }
}

#[cfg(test)]
mod test {

    use super::CallParser;
    use crate::{parser::test_parse, Expression};
    use ark_bn254::Fr;

    pub(crate) fn dummy_path_expr() -> Expression<Fr> {
        use crate::parser::prefix_parser::PrefixParser;

        const SRC: &str = r#"
            std::hash
        "#;

        let mut parser = test_parse(SRC);
        PrefixParser::Path.parse(&mut parser).unwrap()
    }

    #[test]
    fn valid_syntax() {
        let vectors = vec![" ()", " (x,y,a+b)", " (x)", " (x,)"];

        for src in vectors {
            let mut parser = test_parse(src);

            let start = parser.curr_token.clone();

            let _ = CallParser::parse(&mut parser, dummy_path_expr()).unwrap();

            let end = parser.curr_token.clone();

            assert_eq!(start, crate::token::Token::LeftParen);
            assert_eq!(end, crate::token::Token::RightParen);
        }
    }
}
