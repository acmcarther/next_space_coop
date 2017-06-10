use na::Point3;
use na;
use geometry::shape::Cone3;
use procedural::TriMesh3;
use procedural;
use super::ToTriMesh;
use math::Scalar;

impl<N: Scalar> ToTriMesh<Point3<N>, u32> for Cone3<N> {
    fn to_trimesh(&self, nsubdiv: u32) -> TriMesh3<N> {
        // FIXME, inconsistancy we should be able to work directly with the radius.
        // FIXME, inconsistancy we should be able to work directly with the half height.
        let diameter = self.radius() * na::cast(2.0f64);
        let height   = self.half_height() * na::cast(2.0f64);

        procedural::cone(diameter, height, nsubdiv)
    }
}
