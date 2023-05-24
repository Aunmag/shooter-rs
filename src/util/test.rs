macro_rules! assert_radians_eq {
    ($l:expr, $r:expr) => {
        let l = $l;
        let r = $r;
        let difference = crate::util::math::angle_difference(l, r);

        if difference.abs() > 0.00000048 {
            panic!("{} != {} (∆ {})", l, r, difference);
        }
    };
}

pub(crate) use assert_radians_eq;
