use ark_bn254::Fr;
use ark_ff::{to_bytes, Field};
use ark_ff::{BitIteratorBE, PrimeField};
use ark_ff::{One, Zero};
use std::str::FromStr;
// XXX: Switch out for a trait and proper implementations
// This implementation is in-efficient, can definitely remove hex usage and Iterator instances for trivial functionality
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord)]
pub struct FieldElement(Fr);

impl std::hash::Hash for FieldElement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.to_bytes())
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes() == other.to_bytes()
    }
}

impl From<i128> for FieldElement {
    fn from(mut a: i128) -> FieldElement {
        let mut negative = false;
        if a < 0 {
            a = -a;
            negative = true;
        }

        let mut result = Fr::from_str(&a.to_string())
            .expect("Cannot convert i128 as a string to a field element");

        if negative {
            result = -result;
        }
        FieldElement(result)
    }
}

impl FieldElement {
    pub fn one() -> FieldElement {
        FieldElement(Fr::one())
    }
    /// Maximum number of bits needed to represent a field element
    /// This is not the amount of bits being used to represent a field element
    /// Example, you only need 254 bits to represent a field element in BN256
    /// But the representation uses 256 bits, so the top two bits are always zero
    /// This method would return 254
    pub fn max_num_bits() -> u32 {
        // Fr::NUM_BITS
        254
    }
    /// Returns None, if the string is not a canonical
    /// representation of a field element; less than the order
    /// or if the hex string is invalid.
    /// This method can be used for both hex and decimal representations.
    pub fn try_from_str(input: &str) -> Option<FieldElement> {
        if input.contains('x') {
            return FieldElement::from_hex(input);
        }

        let fr = Fr::from_str(input).ok()?;
        Some(FieldElement(fr))
    }
    // This is the amount of bits that are always zero,
    // In BN256, every element can be represented with 254 bits.
    // However this representation uses 256 bits, hence 2 wasted bits
    // Note: This has nothing to do with saturated field elements.
    fn wasted_bits() -> u32 {
        let vec: Vec<_> = BitIteratorBE::new(Fr::one().into_repr()).collect();

        let num_bits_used = vec.len() as u32;
        let num_bits_needed = FieldElement::max_num_bits();
        num_bits_used - num_bits_needed
    }
    pub fn debug_str(&self) -> String {
        self.0.to_string()
    }
    /// This is the number of bits required to represent this specific field element
    pub fn num_bits(&self) -> u32 {
        let non_zero_index = BitIteratorBE::new(self.0.into_repr()).position(|x| x);

        match non_zero_index {
            None => 0,
            Some(index) => {
                // The most significant bit was found at index.
                // The index tells us how many elements came before the most significant bit

                // We need to compute the offset as the representation may have wasted bits
                let offset = FieldElement::wasted_bits();

                // This is now the amount of significant elements that came before the most significant bit
                let msb_index_offset = (index as u32) - offset;

                FieldElement::max_num_bits() - msb_index_offset
            }
        }
    }

    pub fn fits_in_u128(&self) -> bool {
        self.num_bits() <= 128
    }

    pub fn zero() -> FieldElement {
        FieldElement(Fr::zero())
    }
    pub fn is_one(&self) -> bool {
        self == &FieldElement::one()
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    pub fn to_u128(&self) -> u128 {
        use std::convert::TryInto;

        let bytes = self.to_bytes();
        u128::from_be_bytes(bytes[16..32].try_into().unwrap())
    }
    /// Computes the inverse or returns zero if the inverse does not exist
    /// Before using this FieldElement, please ensure that this behaviour is necessary
    pub fn inverse(&self) -> FieldElement {
        let inv = self.0.inverse().unwrap_or_else(Fr::zero);
        FieldElement(inv)
    }

    pub fn to_hex(&self) -> String {
        let mut bytes = to_bytes!(self.0).unwrap();
        bytes.reverse();
        hex::encode(bytes)
    }
    pub fn from_hex(hex_str: &str) -> Option<FieldElement> {
        let dec_str = hex_to_decimal(hex_str);
        Fr::from_str(&dec_str).map(FieldElement).ok()
    }

    // XXX: This is not portable, if the underlying field changes!
    pub fn to_bytes(&self) -> [u8; 32] {
        use std::convert::TryInto;
        hex::decode(self.to_hex()).unwrap().try_into().unwrap()
    }
    /// Converts bytes into a FieldElement. Does not reduce.
    pub fn from_bytes(bytes: &[u8]) -> FieldElement {
        let hex_str = hex::encode(bytes);
        FieldElement::from_hex(&hex_str).unwrap()
    }
    /// Converts bytes into a FieldElement and applies a
    /// reduction if needed.
    pub fn from_bytes_reduce(bytes: &[u8]) -> FieldElement {
        FieldElement(Fr::from_be_bytes_mod_order(bytes))
    }

    // mask_to methods will not remove any bytes from the field
    // they are simply zeroed out
    // Whereas truncate_to will remove those bits and make the byte array smaller
    pub fn mask_to_field(&self, num_bits: u32) -> FieldElement {
        let bit_iter = self.mask_to_bits(num_bits);
        let byte_arr = pack_bits_into_bytes(bit_iter);
        FieldElement::from_bytes(&byte_arr)
    }
    pub fn mask_to_bytes(&self, num_bits: u32) -> Vec<u8> {
        let bit_iter = self.mask_to_bits(num_bits);
        pack_bits_into_bytes(bit_iter)
    }
    pub fn bits(&self) -> Vec<bool> {
        BitIteratorBE::new(self.0.into_repr()).collect()
    }
    fn mask_to_bits(&self, num_bits: u32) -> Vec<bool> {
        let max_bits = FieldElement::max_num_bits() + FieldElement::wasted_bits();

        let bit_iter: Vec<_> = BitIteratorBE::new(self.0.into_repr())
            .enumerate()
            .map(|(i, bit)| {
                if i < (max_bits - num_bits) as usize {
                    false
                } else {
                    bit
                }
            })
            .collect();

        bit_iter
    }
    fn truncate_to_bits(&self, num_bits: u32) -> Vec<bool> {
        let max_bits = FieldElement::max_num_bits() + FieldElement::wasted_bits();

        let bit_iter: Vec<_> = BitIteratorBE::new(self.0.into_repr())
            .enumerate()
            .filter(|(i, _)| *i >= (max_bits - num_bits) as usize)
            .map(|(_, bit)| bit)
            .collect();

        bit_iter
    }
    pub fn truncate_to_bytes(&self, num_bits: u32) -> Vec<u8> {
        let bit_iter = self.truncate_to_bits(num_bits);
        pack_bits_into_bytes(bit_iter)
    }
    /// Returns the closest number of bytes to the bits specified
    pub fn fetch_nearest_bytes(&self, num_bits: usize) -> Vec<u8> {
        fn nearest_bytes(num_bits: usize) -> usize {
            ((num_bits + 7) / 8) * 8
        }

        let num_bytes = nearest_bytes(num_bits);
        let num_elements = num_bytes / 8;

        let mut bytes = self.to_bytes();
        bytes.reverse(); // put it in big endian format. XXX(next refactor): we should be explicit about endianess.

        bytes[0..num_elements].to_vec()
    }

    fn and_xor(&self, rhs: &FieldElement, num_bits: u32, is_xor: bool) -> FieldElement {
        let lhs = self.mask_to_field(num_bits);
        let lhs_bit_iter = BitIteratorBE::new(lhs.0.into_repr());
        let rhs = rhs.mask_to_field(num_bits);
        let rhs_bit_iter = BitIteratorBE::new(rhs.0.into_repr());

        let and_iter: Vec<_> = lhs_bit_iter
            .zip(rhs_bit_iter)
            .map(
                |(bit_a, bit_b)| {
                    if is_xor {
                        bit_a ^ bit_b
                    } else {
                        bit_a & bit_b
                    }
                },
            )
            .collect();

        let byte_arr = pack_bits_into_bytes(and_iter);
        FieldElement::from_bytes(&byte_arr)
    }
    pub fn and(&self, rhs: &FieldElement, num_bits: u32) -> FieldElement {
        self.and_xor(rhs, num_bits, false)
    }
    pub fn xor(&self, rhs: &FieldElement, num_bits: u32) -> FieldElement {
        self.and_xor(rhs, num_bits, true)
    }
}

// Taken from matter-labs: https://github.com/matter-labs/zksync/blob/6bfe1c06f5c00519ce14adf9827086119a50fae2/core/models/src/primitives.rs#L243
fn pack_bits_into_bytes(bits: Vec<bool>) -> Vec<u8> {
    // XXX(FIXME): Passing in just a field element
    // will trigger this panic for bn254.
    // The evaluator will need to pad the number of bits
    // accordingly.
    assert_eq!(
        bits.len() % 8,
        0,
        "input is not a multiple of 8, len is {}",
        bits.len()
    );
    let mut message_bytes: Vec<u8> = vec![];

    let byte_chunks = bits.chunks(8);
    for byte_chunk in byte_chunks {
        let mut byte = 0u8;
        for (i, bit) in byte_chunk.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        message_bytes.push(byte);
    }

    message_bytes
}
// This is needed because arkworks only accepts arbitrary sized
// decimal strings and not hex strings
pub fn hex_to_decimal(value: &str) -> String {
    let value = value.strip_prefix("0x").unwrap_or(value);

    use num_bigint::BigInt;
    BigInt::parse_bytes(value.as_bytes(), 16)
        .unwrap()
        .to_str_radix(10)
}

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

impl Neg for FieldElement {
    type Output = FieldElement;

    fn neg(self) -> Self::Output {
        FieldElement(-self.0)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(mut self, rhs: FieldElement) -> Self::Output {
        use std::ops::MulAssign;
        self.0.mul_assign(&rhs.0);
        FieldElement(self.0)
    }
}
impl Div for FieldElement {
    type Output = FieldElement;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: FieldElement) -> Self::Output {
        self * rhs.inverse()
    }
}
impl Add for FieldElement {
    type Output = FieldElement;
    fn add(mut self, rhs: FieldElement) -> Self::Output {
        self.0.add_assign(&rhs.0);
        FieldElement(self.0)
    }
}
impl AddAssign for FieldElement {
    fn add_assign(&mut self, rhs: FieldElement) {
        self.0.add_assign(&rhs.0);
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(mut self, rhs: FieldElement) -> Self::Output {
        self.0.sub_assign(&rhs.0);
        FieldElement(self.0)
    }
}
impl SubAssign for FieldElement {
    fn sub_assign(&mut self, rhs: FieldElement) {
        self.0.sub_assign(&rhs.0);
    }
}
