use na::{Point2, Point3, Matrix1, Matrix3};
use na;
use ncollide::shape::{Compound2, Compound3};
use ncollide::math::Scalar;
use volumetric::{Volumetric, InertiaTensor};

macro_rules! impl_volumetric_compound(
    ($t: ty, $p: ty, $i: ty) => (
        impl<N: Scalar> Volumetric<N, $p, $i> for $t {
            fn area(&self) -> N {
                let mut stot: N = na::zero();

                for &(_, ref s) in self.shapes().iter() {
                    stot = stot + s.area()
                }

                stot
            }

            fn volume(&self) -> N {
                let mut vtot: N = na::zero();

                for &(_, ref s) in self.shapes().iter() {
                    vtot = vtot + s.volume()
                }

                vtot
            }

            fn center_of_mass(&self) -> $p {
                let mut mtot = na::zero::<N>();
                let mut ctot = na::origin::<$p>();
                let mut gtot = na::origin::<$p>(); // geometric center.

                let shapes = self.shapes();

                for &(ref m, ref s) in shapes.iter() {
                    let (mpart, cpart, _) = s.mass_properties(na::one());

                    mtot = mtot + mpart;
                    ctot = ctot + (*m * cpart * mpart).to_vector();
                    gtot = gtot + (*m * cpart).to_vector();
                }

                if na::is_zero(&mtot) {
                    gtot
                }
                else {
                    ctot / mtot
                }
            }

            fn unit_angular_inertia(&self) -> $i {
                let mut itot = na::zero::<$i>();

                let com    = self.center_of_mass();
                let shapes = self.shapes();

                for &(ref m, ref s) in shapes.iter() {
                    let (mpart, cpart, ipart) = s.mass_properties(na::one());

                    itot = itot + ipart.to_world_space(m)
                                       .to_relative_wrt_point(mpart, &(*m * cpart + (-*com.as_vector())));
                }

                itot
            }

            /// The mass properties of this `CompoundData`.
            ///
            /// If `density` is not zero, it will be multiplied with the density of every object of the
            /// compound shape.
            fn mass_properties(&self, density: N) -> (N, $p, $i) {
                let mut mtot = na::zero::<N>();
                let mut itot = na::zero::<$i>();
                let mut ctot = na::origin::<$p>();
                let mut gtot = na::origin::<$p>(); // geometric center.

                let shapes = self.shapes();
                let props: Vec<_> = shapes.iter().map(|&(_, ref s)| s.mass_properties(na::one())).collect();

                for (&(ref m, _), &(ref mpart, ref cpart, _)) in shapes.iter().zip(props.iter()) {
                    mtot = mtot + *mpart;
                    ctot = ctot + (*m * *cpart * *mpart).to_vector();
                    gtot = gtot + (*m * *cpart).to_vector();
                }

                if na::is_zero(&mtot) {
                    ctot = gtot;
                }
                else {
                    ctot = ctot / mtot;
                }

                for (&(ref m, _), &(ref mpart, ref cpart, ref ipart)) in shapes.iter().zip(props.iter()) {
                    itot = itot + ipart.to_world_space(m).to_relative_wrt_point(*mpart, &(*m * *cpart + (-*ctot.as_vector())));
                }

                (mtot * density, ctot, itot * density)
            }
        }
    )
);

impl_volumetric_compound!(Compound2<N>, Point2<N>, Matrix1<N>);
impl_volumetric_compound!(Compound3<N>, Point3<N>, Matrix3<N>);
