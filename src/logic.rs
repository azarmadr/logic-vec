use core::ops::{
    Not,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXorAssign,
    BitXor,
    BitAnd,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Logic {
    U, // Uninitialized
    X, // Forcing Unkown
    O, // Forcing logic '0'
    I, // Forcing logic '1'
    Z, // High Impedence
    W, // Weak Unkown
    L, // Weak logic '0'
    H, // Weak logic '1'
    Y, // '-' Don't Care logic
}
use Logic::*;

const RESOLVED_T: [Logic; 81] = 
//  U  X  O  I  Z  W  L  H  -
//-------------------------------- 
[
    U, U, U, U, U, U, U, U, U, // U
    U, X, X, X, X, X, X, X, X, // X
    U, X, O, X, O, O, O, O, X, // O
    U, X, X, I, I, I, I, I, X, // I
    U, X, O, I, Z, W, L, H, X, // Z
    U, X, O, I, W, W, W, W, X, // W
    U, X, O, I, L, W, L, W, X, // L
    U, X, O, I, H, W, W, H, X, // H
    U, X, X, X, X, X, X, X, X, // -
];
const AND_T: [Logic; 81] = 
// U  X  O  I  Z  W  L  H  -
// -------------------------- */
[
    U, U, O, U, U, U, O, U, U, // U
    U, X, O, X, X, X, O, X, X, // X
    O, O, O, O, O, O, O, O, O, // O
    U, X, O, I, X, X, O, I, X, // I
    U, X, O, X, X, X, O, X, X, // Z
    U, X, O, X, X, X, O, X, X, // W
    O, O, O, O, O, O, O, O, O, // L
    U, X, O, I, X, X, O, I, X, // H
    U, X, O, X, X, X, O, X, X, // -
];
const OR_T: [Logic; 81] = [/*
                              U  X  O  I  Z  W  L  H  -
                              -------------------------- */
    U, U, U, I, U, U, U, I, U, // U
    U, X, X, I, X, X, X, I, X, // X
    U, X, O, I, X, X, O, I, X, // O
    I, I, I, I, I, I, I, I, I, // I
    U, X, X, I, X, X, X, I, X, // Z
    U, X, X, I, X, X, X, I, X, // W
    U, X, O, I, X, X, O, I, X, // L
    I, I, I, I, I, I, I, I, I, // H
    U, X, X, I, X, X, X, I, X, // -
];
const XOR_T: [Logic; 81] = [/*
                               U  X  O  I  Z  W  L  H  -
                               -------------------------- */
    U, U, U, U, U, U, U, U, U, // U
    U, X, X, X, X, X, X, X, X, // X
    U, X, O, I, X, X, O, I, X, // O
    U, X, I, O, X, X, I, O, X, // I
    U, X, X, X, X, X, X, X, X, // Z
    U, X, X, X, X, X, X, X, X, // W
    U, X, O, I, X, X, O, I, X, // L
    U, X, I, O, X, X, I, O, X, // H
    U, X, X, X, X, X, X, X, X, // -
];
impl Logic {
    pub fn resolved(drv1: Logic, drv2: Logic) -> Self {
        RESOLVED_T[9 * drv1 as usize + drv2 as usize]
    }
}

impl Not for Logic {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            I | H => O,
            O | L => I,
            _ => self,
        }
    }
}

impl BitAnd<Logic> for Logic{
    type Output = Self;

    fn bitand(self, rhs: Logic) -> Self::Output{
        AND_T[9 * self as usize + rhs as usize]
    }
}

impl BitAndAssign<Logic> for Logic{
    fn bitand_assign(&mut self, rhs: Logic) {
        *self = AND_T[9 * *self as usize + rhs as usize]
    }
}

impl BitOr<Logic> for Logic{
    type Output = Self;

    fn bitor(self, rhs: Logic) -> Self::Output{
        OR_T[9 * self as usize + rhs as usize]
    }
}

impl BitOrAssign<Logic> for Logic{
    fn bitor_assign(&mut self, rhs: Logic) {
        *self = OR_T[9 * *self as usize + rhs as usize]
    }
}

impl BitXor<Logic> for Logic{
    type Output = Self;

    fn bitxor(self, rhs: Logic) -> Self::Output{
        XOR_T[9 * self as usize + rhs as usize]
    }
}

impl BitXorAssign<Logic> for Logic{
    fn bitxor_assign(&mut self, rhs: Logic) {
        *self = XOR_T[9 * *self as usize + rhs as usize]
    }
}
