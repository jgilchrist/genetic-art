use std::cmp;

pub fn min3<T>(x: T, y: T, z: T) -> T where T: Ord {
    cmp::min(cmp::min(x, y), z)
}

pub fn max3<T>(x: T, y: T, z: T) -> T where T: Ord {
    cmp::max(cmp::max(x, y), z)
}
