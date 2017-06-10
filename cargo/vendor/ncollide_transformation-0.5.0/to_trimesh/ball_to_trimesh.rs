use na::Point3;
use na;
use geometry::shape::Ball3;
use procedural::TriMesh3;
use procedural;
use super::ToTriMesh;
use math::Scalar;

impl<N: Scalar> ToTriMesh<Point3<N>, (u32, u32)> for Ball3<N> {
    fn to_trimesh(&self, (ntheta_subdiv, nphi_subdiv): (u32, u32)) -> TriMesh3<N> {
        procedural::sphere(self.radius() * na::cast(2.0f64), ntheta_subdiv, nphi_subdiv, true)
    }
}
