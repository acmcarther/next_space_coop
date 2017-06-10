use na::Identity;
use bounding_volume::AABB;
use shape::Compound;
use partitioning::BVTCostFn;
use query::{Ray, RayCast, RayIntersection};
use math::{Point, Vector, Isometry};


// XXX: if solid == false, this might return internal intersection.
impl<P, M> RayCast<P, M> for Compound<P, M>
    where P: Point,
          M: Isometry<P> {
    fn toi_with_ray(&self, m: &M, ray: &Ray<P>, solid: bool) -> Option<<P::Vect as Vector>::Scalar> {
        let ls_ray = Ray::new(m.inverse_transform(&ray.origin), m.inverse_rotate(&ray.dir));

        let mut cost_fn = CompoundRayToiCostFn { compound: self, ray: &ls_ray, solid: solid };

        self.bvt().best_first_search(&mut cost_fn).map(|(_, res)| res)
    }

    fn toi_and_normal_with_ray(&self, m: &M, ray: &Ray<P>, solid: bool) -> Option<RayIntersection<P::Vect>> {
        let ls_ray = Ray::new(m.inverse_transform(&ray.origin), m.inverse_rotate(&ray.dir));

        let mut cost_fn = CompoundRayToiAndNormalCostFn { compound: self, ray: &ls_ray, solid: solid };

        self.bvt().best_first_search(&mut cost_fn).map(|(_, mut res)| {
            res.normal = m.rotate(&res.normal); res
        })
    }

    // XXX: We have to implement toi_and_normal_and_uv_with_ray! Otherwise, no uv will be computed
    // for any of the sub-shapes.
}

/*
 * Costs functions.
 */
struct CompoundRayToiCostFn<'a, P: 'a + Point, M: 'a> {
    compound: &'a Compound<P, M>,
    ray:      &'a Ray<P>,
    solid:    bool
}

impl<'a, P, M> BVTCostFn<<P::Vect as Vector>::Scalar, usize, AABB<P>> for CompoundRayToiCostFn<'a, P, M>
    where P: Point,
          M: Isometry<P> {
    type UserData = <P::Vect as Vector>::Scalar;
    #[inline]
    fn compute_bv_cost(&mut self, aabb: &AABB<P>) -> Option<<P::Vect as Vector>::Scalar> {
        aabb.toi_with_ray(&Identity::new(), self.ray, self.solid)
    }

    #[inline]
    fn compute_b_cost(&mut self, b: &usize) -> Option<(<P::Vect as Vector>::Scalar, <P::Vect as Vector>::Scalar)> {
        let elt = &self.compound.shapes()[*b];
        elt.1.toi_with_ray(&elt.0, self.ray, self.solid).map(|toi| (toi, toi))
    }
}

struct CompoundRayToiAndNormalCostFn<'a, P: 'a + Point, M: 'a> {
    compound: &'a Compound<P, M>,
    ray:      &'a Ray<P>,
    solid:    bool
}

impl<'a, P, M> BVTCostFn<<P::Vect as Vector>::Scalar, usize, AABB<P>>
for CompoundRayToiAndNormalCostFn<'a, P, M>
    where P: Point,
          M: Isometry<P> {
    type UserData = RayIntersection<P::Vect>;

    #[inline]
    fn compute_bv_cost(&mut self, aabb: &AABB<P>) -> Option<<P::Vect as Vector>::Scalar> {
        aabb.toi_with_ray(&Identity::new(), self.ray, self.solid)
    }

    #[inline]
    fn compute_b_cost(&mut self, b: &usize) -> Option<(<P::Vect as Vector>::Scalar, RayIntersection<P::Vect>)> {
        let elt = &self.compound.shapes()[*b];
        elt.1.toi_and_normal_with_ray(&elt.0, self.ray, self.solid).map(|inter| (inter.toi, inter))
    }
}
