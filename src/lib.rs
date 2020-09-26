pub mod drive;
pub mod logic;
#[cfg(test)]
mod tests {
    use crate::drive::*;
    #[test]
    fn it_works() {
        let e1: DriveValue<bool> = DriveValue::Drive(true);
        let e2: DriveValue<bool> = DriveValue::Drive(false);
        let e3: DriveValue<bool> = DriveValue::UnkownValue;
        let mut ex = e1;
        println!("{:?}", ex);
        ex |= e2;
        println!("{:?}", ex);
        println!("{:?} ^ {:?} = {:?}", e1, e1, e1 ^ e1);
        println!("{:?} ^ {:?} = {:?}", e2, e2, e2 ^ e2);
        println!("{:?} ^ {:?} = {:?}", e3, e3, e3 ^ e3);
        println!("{:?} ^ {:?} = {:?}", e1, e2, e1 ^ e2);
        println!("{:?} ^ {:?} = {:?}", e1, e3, e1 ^ e3);
        println!("{:?} ^ {:?} = {:?}", e3, e2, e3 ^ e2);
        println!("{:?} & {:?} = {:?}", e1, e1, e1 & e1);
        println!("{:?} & {:?} = {:?}", e2, e2, e2 & e2);
        println!("{:?} & {:?} = {:?}", e3, e3, e3 & e3);
        println!("{:?} & {:?} = {:?}", e1, e2, e1 & e2);
        println!("{:?} & {:?} = {:?}", e1, e2, e1 & e3);
        println!("{:?} & {:?} = {:?}", e1, e2, e3 & e2);
        println!("{:?} =={:?} = {:?}", e1, e2, e1==e2);
        println!("{:?} =={:?} = {:?}", e1, e2, e1==e3);
        println!("{:?} =={:?} = {:?}", e1, e2, e3==e2);
        println!("{:?} =={:?} = {:?}", e1, e2, e1==e1);
        println!("{:?} =={:?} = {:?}", e1, e2, e2==e2);
        println!("{:?} =={:?} = {:?}", e1, e2, e3==e3);
    }
}
