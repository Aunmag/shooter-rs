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

macro_rules! assert_vec_is_close {
    ($v1:expr, $v2:expr, $t:expr) => {
        let v1 = $v1;
        let v2 = $v2;
        let delta = (v1 - v2).length() / (v1.length() + v2.length());

        if delta > $t {
            panic!("{} and {} are too far off ({}%)", v1, v2, delta * 100.0);
        }
    };
}

pub(crate) use assert_radians_eq;
pub(crate) use assert_vec_is_close;
