use num::traits::Num;
use num::NumCast;

pub fn cantor_pairing<T: Copy + Num + NumCast, U: Into<T>>(x: U, y: U) -> T {
    let nx: T = x.into();
    let ny: T = y.into();
    ((nx + ny) * (nx + ny + NumCast::from(1).unwrap())) / NumCast::from(2).unwrap() + ny
}
