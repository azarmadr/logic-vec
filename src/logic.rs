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
    U, X, W, Any, Sd(bool), Wd(bool),
}

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
                Logic::Sd(true) => Logic::Sd(true),
                Logic::Wd(true) => Logic::Sd(true),
                _ => self,
            }
            Logic::X => match rhs {
                Logic::U => rhs,
                Logic::X => rhs,
                Logic::Sd(true) => rhs,
                Logic::Wd(true) => Logic::Sd(true),
                _ => self,
            }
            Logic::W | Logic::Any => match rhs {
                Logic::U => rhs,
                Logic::Sd(true) => rhs,
                Logic::Wd(true) => Logic::Sd(true),
                _ => Logic::X,
            }
            Logic::Sd(true) | Logic::Wd(true)=> Logic::Sd(true),
            Logic::Sd(false) | Logic::Wd(false)=> match rhs {
                Logic::W | Logic::Any => Logic::X,
                Logic::Wd(true) => Logic::Sd(true),
                Logic::Wd(false) => Logic::Sd(false),
                _ => rhs,
            }
        }
    }
}
