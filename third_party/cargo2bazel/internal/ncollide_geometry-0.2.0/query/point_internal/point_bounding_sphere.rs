use na::{Identity, Transform};
use query::{PointQuery, PointProjection};
use shape::Ball;
use bounding_volume::BoundingSphere;
use math::{Point, Vector};

impl<P, M> PointQuery<P, M> for BoundingSphere<P>
    where P: Point,
          M: Transform<P> {
    #[inline]
    fn project_point(&self, m: &M, pt: &P, solid: bool) -> PointProjection<P> {
        let ls_pt = m.inverse_transform(pt) + (-*self.center().as_vector());
        let mut proj = Ball::new(self.radius()).project_point(&Identity::new(), &ls_pt, solid);

        proj.point = m.transform(&proj.point) + *self.center().as_vector();

        proj
    }

    #[inline]
    fn distance_to_point(&self, m: &M, pt: &P, solid: bool) -> <P::Vect as Vector>::Scalar {
        let ls_pt = m.inverse_transform(pt) + (-*self.center().as_vector());

        Ball::new(self.radius()).distance_to_point(&Identity::new(), &ls_pt, solid)
    }

    #[inline]
    fn contains_point(&self, m: &M, pt: &P) -> bool {
        let ls_pt = m.inverse_transform(pt) + (-*self.center().as_vector());

        Ball::new(self.radius()).contains_point(&Identity::new(), &ls_pt)
    }
}
