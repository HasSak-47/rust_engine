
macro_rules! impl_op{
    ($op: tt, $op_n: tt, $for_type: ty, $o_type: ty, $out_type: ty, $oper: block, $self: tt, $rhs: tt) => {
        impl<T> $op<$o_type> for $for_type
        where
            T: Add<T, Output = T> +
               Sub<T, Output = T> +
               Mul<T, Output = T> +
               Div<T, Output = T> +
               Neg<   Output = T> +
               Copy + Clone
        {
            type Output = $out_type;
            fn $op_n($self, $rhs: $o_type) -> $out_type { $oper }
        }
    }
}

macro_rules! impl_ass{
    ($op: tt, $op_n: tt, $for_type: ty, $o_type: ty, $oper: block, $self: tt, $rhs: tt) => {
        impl<T> $op<$o_type> for $for_type
        where
            T: Add<T, Output = T> +
               Sub<T, Output = T> +
               Mul<T, Output = T> +
               Div<T, Output = T> +
               Neg<   Output = T> +
               Copy + Clone,
        {
            // type Output = $for_type;
            fn $op_n(&mut $self, $rhs: $o_type) { $oper }
        }
    }
}

pub(super) use impl_op;
pub(super) use impl_ass;
