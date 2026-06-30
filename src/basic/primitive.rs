macro_rules! map_binary_operator {
    (Self, $a:expr, $b:expr, $op:expr) => {
        $a.into_iter_fixed()
            .zip($b)
            .map(|(a, b)| $op(a, b))
            .collect::<[f64; 3]>()
            .into()
    };
    (f64, $a:expr, $b:expr, $op:expr) => {
        $a.into_iter_fixed()
            .map(|a| $op(a, $b))
            .collect::<[f64; 3]>()
            .into()
    };
}

macro_rules! map_assign_operator {
    (Self, $a:expr, $b:expr, $op:expr) => {
        $a.iter_mut().zip($b).for_each(|(a, b)| $op(a, b));
    };
    (f64, $a:expr, $b:expr, $op:expr) => {
        $a.into_iter_fixed()
            .map(|a| $op(a, $b))
            .collect::<[f64; 3]>()
            .into()
    };
}

macro_rules! primitive_operators {
    (
        $primitive:ty;
        $(
            [rhs=$rhs_type:tt] {
                $(
                    $trait:ident($method:ident) $(+ $a_trait:ident($a_method:ident))?
                );+;
            }
        )+
    ) => (
        $($(
            impl $trait<$rhs_type> for $primitive {
                type Output = Self;
                fn $method(self, rhs: $rhs_type) -> Self {
                    map_binary_operator!($rhs_type, self, rhs, f64::$method)
                }
            }
            $(
                impl $a_trait for $primitive {
                    fn $a_method(&mut self, rhs: Self) {
                        *self = (*self).$method(rhs);
                    }
                }
            )?
        )+)+
    );
}

pub(crate) use {map_binary_operator, primitive_operators};
