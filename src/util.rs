macro_rules! unwrap_or_return {
    ( $v:expr, $e:expr ) => {
        match $v {
            Ok(x) => x,
            Err(_) => return $e,
        }
    };
}

pub(crate) use unwrap_or_return;

pub type Vec2D<T> = Vec<Vec<T>>;
