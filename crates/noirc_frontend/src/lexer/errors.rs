use super::token::Token;
use noir_field::FieldElement;
use noirc_errors::CustomDiagnostic as Diagnostic;
use noirc_errors::{DiagnosableError, Span};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerErrorKind<F: std::fmt::Debug> {
    #[error("An unexpected character {:?} was found.", found)]
    UnexpectedCharacter { span: Span, found: char },
    #[error("The character {:?} is not in the language.", found)]
    CharacterNotInLanguage { span: Span, found: char },
    #[error("Internal Error : {:?} is not a double char token", found)]
    NotADoubleChar { span: Span, found: Token<F> },
}

impl<F: FieldElement> DiagnosableError for LexerErrorKind<F> {
    fn to_diagnostic(&self) -> Diagnostic {
        match self {
            LexerErrorKind::UnexpectedCharacter { span, found } => Diagnostic::simple_error(
                "an unexpected character was found".to_string(),
                format!(" {:?} is unexpected", found),
                *span,
            ),
            LexerErrorKind::CharacterNotInLanguage { span, found } => Diagnostic::simple_error(
                "char is not in language".to_string(),
                format!(" {:?} is not in language", found),
                *span,
            ),
            LexerErrorKind::NotADoubleChar { span, found } => Diagnostic::simple_error(
                format!("tried to parse {} as double char", found),
                format!(
                    " {:?} is not a double char, this is an internal error",
                    found
                ),
                *span,
            ),
        }
    }
}
