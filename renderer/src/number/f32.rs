use std::cmp::Ordering;

pub fn total_cmp(x: &f32, y: &f32) -> Ordering {
    if x.is_nan() && y.is_nan() {
        Ordering::Equal
    } else if (x.is_nan() && !y.is_nan()) || x > y {
        Ordering::Greater
    } else if (!x.is_nan() && y.is_nan()) || x < y {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
