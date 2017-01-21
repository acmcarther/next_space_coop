use num::Zero;
use na::{self, Translation, Translate};
use shape::{self, SupportMap, AnnotatedPoint};
use query::algorithms::gjk;
use query::algorithms::simplex::Simplex;
use query::algorithms::johnson_simplex::JohnsonSimplex;
use query::Proximity;
use math::{Point, Vector};


/// Proximity between support-mapped shapes (`Cuboid`, `ConvexHull`, etc.)
pub fn support_map_against_support_map<P, M, G1: ?Sized, G2: ?Sized>(
                                       m1:         &M,
                                       g1:         &G1,
                                       m2:         &M,
                                       g2:         &G2,
                                       margin: <P::Vect as Vector>::Scalar)
                                       -> Proximity
    where P:  Point,
          P::Vect: Translate<P>,
          M:  Translation<P::Vect>,
          G1: SupportMap<P, M>,
          G2: SupportMap<P, M> {
    support_map_against_support_map_with_params(m1, g1, m2, g2, margin, &mut JohnsonSimplex::new_w_tls(), None).0
}

/// Proximity between support-mapped shapes (`Cuboid`, `ConvexHull`, etc.)
///
/// This allows a more fine grained control other the underlying GJK algorigtm.
pub fn support_map_against_support_map_with_params<P, M, S, G1: ?Sized, G2: ?Sized>(
                                                   m1:       &M,
                                                   g1:       &G1,
                                                   m2:       &M,
                                                   g2:       &G2,
                                                   margin:   <P::Vect as Vector>::Scalar,
                                                   simplex:  &mut S,
                                                   init_dir: Option<P::Vect>)
                                                   -> (Proximity, P::Vect)
    where P:  Point,
          P::Vect: Translate<P>,
          M:  Translation<P::Vect>,
          S:  Simplex<AnnotatedPoint<P>>,
          G1: SupportMap<P, M>,
          G2: SupportMap<P, M> {
    assert!(margin >= na::zero(), "The proximity margin must be positive or null.");

    let mut dir =
        match init_dir {
            None      => m1.translation() - m2.translation(), // FIXME: or m2.translation - m1.translation ?
            Some(dir) => dir
        };

    if dir.is_zero() {
        dir[0] = na::one();
    }

    simplex.reset(shape::cso_support_point(m1, g1, m2, g2, dir));

    gjk::proximity(m1, g1, m2, g2, margin, simplex)
}
