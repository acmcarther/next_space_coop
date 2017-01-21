//! Explicit Euler integrator.

use ncollide::math::Scalar;
use na::Transformation;
use object::RigidBody;
use integration::Integrator;
use integration::euler;

/// An explicit Euler integrator.
///
/// Do not use this, prefer the `BodySmpEulerIntegrator` instead.
pub struct BodyExpEulerIntegrator;

impl BodyExpEulerIntegrator {
    /// Creates a new `BodyExpEulerIntegrator`.
    #[inline]
    pub fn new() -> BodyExpEulerIntegrator {
        BodyExpEulerIntegrator
    }
}

impl<N: Scalar> Integrator<N, RigidBody<N>> for BodyExpEulerIntegrator {
    #[inline]
    fn update(&mut self, dt: N, rb: &mut RigidBody<N>) {
        if rb.can_move() {
            let (t, lv, av) = euler::explicit_integrate(
                dt.clone(),
                rb.position(),
                rb.center_of_mass(),
                &rb.lin_vel(),
                &rb.ang_vel(),
                &rb.lin_acc(),
                &rb.ang_acc());

            rb.append_transformation(&t);
            rb.set_lin_vel(lv);
            rb.set_ang_vel(av);
        }
    }
}
