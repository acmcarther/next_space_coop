use na::Point3;
use na;
use geometry::shape::Cylinder3;
use procedural::TriMesh3;
use procedural;
use super::ToTriMesh;
use math::Scalar;

impl<N: Scalar> ToTriMesh<Point3<N>, u32> for Cylinder3<N> {
    fn to_trimesh(&self, nsubdiv: u32) -> TriMesh3<N> {
        let diameter = self.radius() * na::cast(2.0f64);
        let height   = self.half_height() * na::cast(2.0f64);

        procedural::cylinder(diameter, height, nsubdiv)
    }
}
