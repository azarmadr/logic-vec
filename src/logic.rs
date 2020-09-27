use crate::drive::*;

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
    U, X, W, Z, Any, Sd(bool), Wd(bool),
}

impl Logic {
    pub fn zero() -> Self {
        Logic::Sd(false)
    }
    pub fn resolved(drv1: Logic, drv2: Logic) -> Self {
        match drv1 {
            Logic::U => Logic::U,
            Logic::X |Logic::Any => match drv2 {
                Logic::U => drv2,
                _ => Logic::X,
            }
            Logic::Sd(true) => match drv2 {
                Logic::U => Logic::U,
                Logic::Sd(false) | Logic::X |Logic::Any => Logic::X,
                _ => drv1,
            }
            Logic::Sd(false) => match drv2 {
                Logic::U => Logic::U,
                Logic::Sd(true) | Logic::X |Logic::Any => Logic::X,
                _ => drv1,
            }
            Logic::Z => drv2,
            Logic::W => match drv2 {
                Logic::U | Logic::X => drv2,
                Logic::Sd(x) => Logic::Sd(x),
                Logic::Any => Logic::X,
                _ => Logic::W,
            }
            Logic::Wd(true) => match drv2 {
                Logic::Wd(false) => Logic::W,
                Logic::Z => drv1,
                Logic::Any => Logic::X,
                _ => drv2,
            }
            Logic::Wd(false) => match drv2 {
                Logic::Wd(true) => Logic::W,
                Logic::Z => drv1,
                Logic::Any => Logic::X,
                _ => drv2,
            }
        }
    }
}
/*     U   X   0   1   Z   W   L   H   - 
L -- (‘U’,‘X’,‘0’,‘1’,‘L’,‘W’,‘L’,‘W’,‘X’)
H -- (‘U’,‘X’,‘0’,‘1’,‘H’,‘W’,‘W’,‘H’,‘X’)
*/
impl Not for Logic{
    type Output = Self;

    fn not(self) -> Self::Output{
        match self{
            Logic::Sd(x) => Logic::Sd(!x),
            Logic::Wd(x) => Logic::Wd(!x),
            _ => self,
        }
    }
}
impl BitAnd<Logic> for Logic{
    type Output = Self;

    fn bitand(self, rhs: Logic) -> Self::Output{
        match self{
            Logic::U=> match rhs {
                Logic::Sd(false) => Logic::Sd(false),
                Logic::Wd(false) => Logic::Sd(false),
                _ => self,
            }
            Logic::Z |Logic::X | Logic::W | Logic::Any => match rhs {
                Logic::U => rhs,
                Logic::Sd(false) => rhs,
                Logic::Wd(false) => Logic::Sd(false),
                _ => Logic::X,
            }
            Logic::Sd(false) | Logic::Wd(false)=> Logic::Sd(false),
            Logic::Sd(true) | Logic::Wd(true)=> match rhs {
                Logic::W | Logic::Any => Logic::X,
                Logic::Wd(x) => Logic::Sd(x),
                _ => rhs,
            }
        }
    }
}
impl BitAndAssign<Logic> for Logic{
    fn bitand_assign(&mut self, rhs: Logic) {
        *self = *self & rhs
    }
}
impl BitOr<Logic> for Logic{
    type Output = Self;

    fn bitor(self, rhs: Logic) -> Self::Output{
        match self{
            Logic::U=> match rhs {
                Logic::Sd(true) => Logic::Sd(true),
                Logic::Wd(true) => Logic::Sd(true),
                _ => self,
            }
            Logic::Z |Logic::X | Logic::W | Logic::Any => match rhs {
                Logic::U => rhs,
                Logic::Sd(true) => rhs,
                Logic::Wd(true) => Logic::Sd(true),
                _ => Logic::X,
            }
            Logic::Sd(true) | Logic::Wd(true)=> Logic::Sd(true),
            Logic::Sd(false) | Logic::Wd(false)=> match rhs {
                Logic::W | Logic::Any => Logic::X,
                Logic::Wd(x) => Logic::Sd(x),
                _ => rhs,
            }
        }
    }
}
impl BitOrAssign<Logic> for Logic{
    fn bitor_assign(&mut self, rhs: Logic) {
        *self = *self | rhs
    }
}
impl BitXor<Logic> for Logic{
    type Output = Self;

    fn bitxor(self, rhs: Logic) -> Self::Output{
        match self{
            Logic::U=> self,
            Logic::Z |Logic::X | Logic::W | Logic::Any => match rhs {
                Logic::U => rhs,
                _ => Logic::X,
            }
            Logic::Sd(x) | Logic::Wd(x)=> match rhs {
                Logic::U => rhs,
                Logic::X | Logic::W | Logic::Any => Logic::X,
                Logic::Wd(y) | Logic::Sd(y) => Logic::Sd(x^y),
            }
        }
    }
}
impl BitXorAssign<Logic> for Logic{
    fn bitxor_assign(&mut self, rhs: Logic) {
        *self = *self ^ rhs
    }
}
