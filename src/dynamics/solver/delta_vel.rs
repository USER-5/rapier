use crate::math::{AngVector, Real, Vector};
use na::{Scalar, SimdRealField};

#[derive(Copy, Clone, Debug)]
//#[repr(align(64))]
pub(crate) struct DeltaVel<N: Scalar + Copy> {
    pub linear: Vector<N>,
    pub angular: AngVector<N>,
}

impl<N: SimdRealField> DeltaVel<N> {
    pub fn zero() -> Self {
        Self {
            linear: na::zero(),
            angular: na::zero(),
        }
    }
}
