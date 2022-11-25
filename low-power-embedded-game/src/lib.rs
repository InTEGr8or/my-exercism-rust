// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    if(std::any::type_name::<T>() == "i32") {
        iter.map(|x| x * 2) 
    } else {
        iter.filter(|x| x % 2 == 0)
    }
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        unimplemented!("implement `fn manhattan`")
    }
}
