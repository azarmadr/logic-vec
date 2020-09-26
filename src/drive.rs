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
pub enum DriveValue<T> {
    Drive(T),
    UnkownValue,
}

impl Not for DriveValue<bool>{
    type Output = Self;

    fn not(self) -> Self::Output{
        match self{
            DriveValue::Drive(true) => DriveValue::Drive(false),
            DriveValue::Drive(false) => DriveValue::Drive(true),
            _ => self,
        }
    }
}

impl BitAnd<DriveValue<bool>> for DriveValue<bool>{
    type Output = Self;
    fn bitand(self, rhs: DriveValue<bool>) -> Self::Output{
        match self {
            DriveValue::Drive(false) => DriveValue::Drive(false),
            DriveValue::UnkownValue => match rhs {
                DriveValue::Drive(false) => DriveValue::Drive(false),
                _ => DriveValue::UnkownValue,
            }
            DriveValue::Drive(true) => rhs,
        }
    }
}

impl BitAndAssign<DriveValue<bool>> for DriveValue<bool>{
    fn bitand_assign(&mut self, rhs: DriveValue<bool>) {
        *self = *self & rhs
    }
}
impl BitOr<DriveValue<bool>> for DriveValue<bool>{
    type Output = Self;
    fn bitor(self, rhs: DriveValue<bool>) -> Self::Output{
        match self {
            DriveValue::Drive(true) => DriveValue::Drive(true),
            DriveValue::UnkownValue => match rhs {
                DriveValue::Drive(true) => DriveValue::Drive(true),
                _ => DriveValue::UnkownValue,
            }
            DriveValue::Drive(false) => rhs,
        }
    }
}

impl BitOrAssign<DriveValue<bool>> for DriveValue<bool>{
    fn bitor_assign(&mut self, rhs: DriveValue<bool>) {
        *self = *self | rhs
    }
}

impl BitXor<DriveValue<bool>> for DriveValue<bool>{
    type Output = Self;
    fn bitxor(self, rhs: DriveValue<bool>) -> Self::Output{
        (self & !rhs) | (!self & rhs)
    }
}

impl BitXorAssign<DriveValue<bool>> for DriveValue<bool>{
    fn bitxor_assign(&mut self, rhs: DriveValue<bool>) {
        *self = *self ^ rhs
    }
}
