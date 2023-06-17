macro_rules! ok_or_continue {
    ($e:expr) => {
        if let Ok(value) = $e {
            value
        } else {
            continue;
        }
    };
}

pub(crate) use ok_or_continue;
