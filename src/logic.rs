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
pub enum LogicValue<T> {
    Uninitialized,
    StrongDrive(DriveValue<T>),
    WeakDrive(DriveValue<T>),
    HigeImpedence,
    DontCare,
}

impl Not for LogicValue<bool>{
    type Output = Self;

    fn not(self) -> Self::Output{
        match self{
            LogicValue::StrongDrive(x) => LogicValue::StrongDrive(!x),
            LogicValue::WeakDrive(x) => LogicValue::WeakDrive(!x),
            _ => self,
        }
    }
}
