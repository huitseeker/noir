// You can only do bit operations with integers.
// (Kobi/Daira/Circom/#37) https://github.com/iden3/circom/issues/37
//
//
fn main(x : Field, y : Field) {
    priv x_as_u8 = x as u8;
    priv y_as_u8 = y as u8;
    
    constrain (x_as_u8 & y_as_u8) == y;
}
    