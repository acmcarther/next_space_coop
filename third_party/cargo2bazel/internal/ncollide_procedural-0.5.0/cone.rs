use na;
use na::{Point3, Vector3, BaseFloat};
use super::{TriMesh, IndexBuffer};
use super::utils;
use math::Scalar;

/// Generates a cone with a given height and diameter.
pub fn cone<N>(diameter: N, height: N, nsubdiv: u32) -> TriMesh<Point3<N>>
    where N: Scalar {
    let mut cone = unit_cone(nsubdiv);

    cone.scale_by(&Vector3::new(diameter, height, diameter));

    cone
}

/// Generates a cone with unit height and diameter.
pub fn unit_cone<N>(nsubdiv: u32) -> TriMesh<Point3<N>>
    where N: Scalar {
    let two_pi: N   = BaseFloat::two_pi();
    let dtheta      = two_pi / na::cast(nsubdiv as f64);
    let mut coords  = Vec::new();
    let mut indices = Vec::new();
    let mut normals: Vec<Vector3<N>>;

    utils::push_circle(na::cast(0.5), nsubdiv, dtheta, na::cast(-0.5), &mut coords);

    normals = coords.iter().map(|p| p.as_vector().clone()).collect();

    coords.push(Point3::new(na::zero(), na::cast(0.5), na::zero()));

    utils::push_degenerate_top_ring_indices(0, coords.len() as u32 - 1, nsubdiv, &mut indices);
    utils::push_filled_circle_indices(0, nsubdiv, &mut indices);

    /*
     * Normals.
     */
    let mut indices = utils::split_index_buffer(&indices[..]);

    // adjust the normals:
    let shift: N = na::cast(0.05 / 0.475);
    let div = (shift * shift + na::cast(0.25)).sqrt();
    for n in normals.iter_mut() {
        n.y = n.y + shift;
        // FIXME: n / div does not work?
        n.x = n.x / div;
        n.y = n.y / div;
        n.z = n.z / div;
    }

    // normal for the basis
    normals.push(Vector3::new(na::zero(), -na::one::<N>(), na::zero()));

    let ilen = indices.len();
    let nlen = normals.len() as u32;
    for (id, i) in indices[.. ilen - (nsubdiv as usize - 2)].iter_mut().enumerate() {
        i.y.y = id as u32;
    }

    for i in indices[ilen - (nsubdiv as usize - 2) ..].iter_mut() {
        i.x.y = nlen - 1;
        i.y.y = nlen - 1;
        i.z.y = nlen - 1;
    }

    // normal for the body

    TriMesh::new(coords, Some(normals), None, Some(IndexBuffer::Split(indices)))

    // XXX: uvs
}
