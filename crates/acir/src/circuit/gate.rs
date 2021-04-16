use crate::native_types::{Arithmetic, Witness};
use crate::OPCODE;

#[derive(Clone, Debug)]
pub struct AndGate {
    pub a: Witness,
    pub b: Witness,
    pub result: Witness,
    pub num_bits: u32,
}

#[derive(Clone, Debug)]
pub struct XorGate {
    pub a: Witness,
    pub b: Witness,
    pub result: Witness,
    pub num_bits: u32,
}

#[derive(Clone, Debug)]
// XXX: Gate does not capture what this is anymore. I think IR/OPCODE would be a better name
pub enum Gate<F> {
    Arithmetic(Arithmetic<F>),
    Range(Witness, u32),
    And(AndGate),
    Xor(XorGate),
    GadgetCall(GadgetCall),
    Directive(Directive),
}

impl<F> Gate<F> {
    pub fn is_arithmetic(&self) -> bool {
        matches!(self, Gate::Arithmetic(_))
    }
    pub fn arithmetic(self) -> Arithmetic<F> {
        match self {
            Gate::Arithmetic(gate) => gate,
            _ => panic!("tried to convert a non arithmetic gate to an Arithmetic struct"),
        }
    }
}

#[derive(Clone, Debug)]
/// Directives do not apply any constraints.
pub enum Directive {
    Invert { x: Witness, result: Witness },
}

// Note: Some gadgets will not use all of the witness
// So we need to supply how many bits of the witness is needed
#[derive(Clone, Debug)]
pub struct GadgetInput {
    pub witness: Witness,
    pub num_bits: u32,
}

#[derive(Clone, Debug)]
pub struct GadgetCall {
    pub name: OPCODE,
    pub inputs: Vec<GadgetInput>,
    pub outputs: Vec<Witness>,
}
