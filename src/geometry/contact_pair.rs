use crate::dynamics::{BodyPair, RigidBodySet};
use crate::geometry::{ColliderPair, ContactManifold};
use crate::math::{Point, Real, Vector};
use cdl::query::ContactManifoldsWorkspace;

bitflags::bitflags! {
    #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
    /// Flags affecting the behavior of the constraints solver for a given contact manifold.
    pub struct SolverFlags: u32 {
        /// The constraint solver will take this contact manifold into
        /// account for force computation.
        const COMPUTE_IMPULSES = 0b01;
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
/// A single contact between two collider.
pub struct ContactData {
    /// The impulse, along the contact normal, applied by this contact to the first collider's rigid-body.
    ///
    /// The impulse applied to the second collider's rigid-body is given by `-impulse`.
    pub impulse: Real,
    /// The friction impulse along the vector orthonormal to the contact normal, applied to the first
    /// collider's rigid-body.
    #[cfg(feature = "dim2")]
    pub tangent_impulse: Real,
    /// The friction impulses along the basis orthonormal to the contact normal, applied to the first
    /// collider's rigid-body.
    #[cfg(feature = "dim3")]
    pub tangent_impulse: [Real; 2],
}

impl ContactData {
    #[cfg(feature = "dim2")]
    pub(crate) fn zero_tangent_impulse() -> Real {
        0.0
    }

    #[cfg(feature = "dim3")]
    pub(crate) fn zero_tangent_impulse() -> [Real; 2] {
        [0.0, 0.0]
    }
}

impl Default for ContactData {
    fn default() -> Self {
        Self {
            impulse: 0.0,
            tangent_impulse: Self::zero_tangent_impulse(),
        }
    }
}

#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[derive(Clone)]
/// The description of all the contacts between a pair of colliders.
pub struct ContactPair {
    /// The pair of colliders involved.
    pub pair: ColliderPair,
    /// The set of contact manifolds between the two colliders.
    ///
    /// All contact manifold contain themselves contact points between the colliders.
    pub manifolds: Vec<ContactManifold>,
    pub has_any_active_contact: bool,
    pub(crate) workspace: Option<ContactManifoldsWorkspace>,
}

impl ContactPair {
    pub(crate) fn new(pair: ColliderPair) -> Self {
        Self {
            pair,
            has_any_active_contact: false,
            manifolds: Vec::new(),
            workspace: None,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
/// A contact manifold between two colliders.
///
/// A contact manifold describes a set of contacts between two colliders. All the contact
/// part of the same contact manifold share the same contact normal and contact kinematics.
pub struct ContactManifoldData {
    // The following are set by the narrow-phase.
    /// The pair of body involved in this contact manifold.
    pub body_pair: BodyPair,
    pub(crate) warmstart_multiplier: Real,
    // The two following are set by the constraints solver.
    pub(crate) constraint_index: usize,
    pub(crate) position_constraint_index: usize,
    // We put the following fields here to avoids reading the colliders inside of the
    // contact preparation method.
    /// Flags used to control some aspects of the constraints solver for this contact manifold.
    pub solver_flags: SolverFlags,
    pub normal: Vector<Real>,
    pub solver_contacts: Vec<SolverContact>,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct SolverContact {
    pub point: Point<Real>,
    pub dist: Real,
    pub friction: Real,
    pub restitution: Real,
    pub surface_velocity: Vector<Real>,
    pub data: ContactData,
}

impl Default for ContactManifoldData {
    fn default() -> Self {
        Self::new(
            BodyPair::new(
                RigidBodySet::invalid_handle(),
                RigidBodySet::invalid_handle(),
            ),
            SolverFlags::empty(),
        )
    }
}

impl ContactManifoldData {
    pub(crate) fn new(body_pair: BodyPair, solver_flags: SolverFlags) -> ContactManifoldData {
        Self {
            body_pair,
            warmstart_multiplier: Self::min_warmstart_multiplier(),
            constraint_index: 0,
            position_constraint_index: 0,
            solver_flags,
            normal: Vector::zeros(),
            solver_contacts: Vec::new(),
        }
    }

    #[inline]
    pub fn num_active_contacts(&self) -> usize {
        self.solver_contacts.len()
    }

    pub(crate) fn min_warmstart_multiplier() -> Real {
        // Multiplier used to reduce the amount of warm-starting.
        // This coefficient increases exponentially over time, until it reaches 1.0.
        // This will reduce significant overshoot at the timesteps that
        // follow a timestep involving high-velocity impacts.
        1.0 // 0.01
    }

    // pub(crate) fn update_warmstart_multiplier(manifold: &mut ContactManifold) {
    //     // In 2D, tall stacks will actually suffer from this
    //     // because oscillation due to inaccuracies in 2D often
    //     // cause contacts to break, which would result in
    //     // a reset of the warmstart multiplier.
    //     if cfg!(feature = "dim2") {
    //         manifold.data.warmstart_multiplier = 1.0;
    //         return;
    //     }
    //
    //     for pt in &manifold.points {
    //         if pt.data.impulse != 0.0 {
    //             manifold.data.warmstart_multiplier =
    //                 (manifold.data.warmstart_multiplier * 2.0).min(1.0);
    //             return;
    //         }
    //     }
    //
    //     // Reset the multiplier.
    //     manifold.data.warmstart_multiplier = Self::min_warmstart_multiplier()
    // }
}
