/// Describes a Y movement pattern as a function of T.
/// Evaluates to y = m*f_inner(t * b)
pub struct MoveFunction {
    m: usize,
    f_inner: InnerFunc,
    b: usize,
}

pub enum InnerFunc {
    Sin,
    Cos,
}
