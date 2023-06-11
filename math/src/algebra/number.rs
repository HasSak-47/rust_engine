
pub trait Number{}
pub trait Float where Self: Number {}
pub trait Integer where Self: Number{}
