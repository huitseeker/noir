use crate::{token::Attribute, Ident};

use super::{FunctionDefinition, Type};

// A NoirFunction can be either a foreign low level function or a function definition
// A closure / function definition will be stored under a name, so we do not differentiate between their variants
// The name for function literal will be the variable it is binded to, and the name for a function definition will
// be the function name itself.
#[derive(Clone, Debug, PartialEq)]
pub struct NoirFunction {
    pub kind: FunctionKind,
    pub def: FunctionDefinition,
}

/// Currently, we support three types of functions:
/// - Normal functions
/// - LowLevel/Foreign which link to an OPCODE in ACIR
/// - BuiltIn which are provided by the runtime
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FunctionKind {
    LowLevel,
    Builtin,
    Normal,
}

impl NoirFunction {
    pub fn normal(def: FunctionDefinition) -> NoirFunction {
        NoirFunction {
            kind: FunctionKind::Normal,
            def,
        }
    }
    pub fn builtin(def: FunctionDefinition) -> NoirFunction {
        NoirFunction {
            kind: FunctionKind::Builtin,
            def,
        }
    }
    pub fn low_level(def: FunctionDefinition) -> NoirFunction {
        NoirFunction {
            kind: FunctionKind::LowLevel,
            def,
        }
    }

    pub fn return_type(&self) -> Type {
        self.def.return_type.clone()
    }
    pub fn name(&self) -> &str {
        &self.name_ident().0.contents
    }
    pub fn name_ident(&self) -> &Ident {
        &self.def.name
    }
    pub fn parameters(&self) -> &Vec<(Ident, Type)> {
        &self.def.parameters
    }
    pub fn attribute(&self) -> Option<&Attribute> {
        self.def.attribute.as_ref()
    }
    pub fn def(&self) -> &FunctionDefinition {
        &self.def
    }
    pub fn def_mut(&mut self) -> &mut FunctionDefinition {
        &mut self.def
    }
    pub fn number_of_statements(&self) -> usize {
        self.def.body.0.len()
    }

    pub fn foreign(&self) -> Option<&FunctionDefinition> {
        match &self.kind {
            FunctionKind::LowLevel => {}
            _ => return None,
        }
        assert!(self.attribute().unwrap().is_foreign());
        Some(&self.def)
    }
}

impl Into<NoirFunction> for FunctionDefinition {
    fn into(self) -> NoirFunction {
        let kind = match self.attribute {
            Some(Attribute::Builtin(_)) => FunctionKind::Builtin,
            Some(Attribute::Foreign(_)) => FunctionKind::LowLevel,
            None => FunctionKind::Normal,
        };

        NoirFunction { def: self, kind }
    }
}
