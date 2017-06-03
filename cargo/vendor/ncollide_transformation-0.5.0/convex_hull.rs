use std::cmp::Ordering;
use std::ops::{Mul, Add};
use std::marker::PhantomData;
use num::Zero;
use na::{Identity, Point2, Point3, Norm, Column, Diagonal, Outer, EigenQR, Bounded};
use na;
use math::{Point, Vector, FloatError};
use utils;
use geometry::bounding_volume;
use procedural::{Polyline, TriMesh, IndexBuffer};

/*
 * XXX: when conditional dispatch is supported by rustc, create a generic convex hull function
 * for both 2D and 3D.
 */

// FIXME: factorize with the one on hacd.
fn normalize<P>(coords: &mut [P]) -> (P, <P::Vect as Vector>::Scalar)
    where P: Point {
    let (mins, maxs) = bounding_volume::point_cloud_aabb(&Identity::new(), &coords[..]);
    let diag   = na::distance(&mins, &maxs);
    let center = na::center(&mins, &maxs);

    for c in coords.iter_mut() {
        *c = (*c + (-*center.as_vector())) / diag;
    }

    (center, diag)
}

fn denormalize<P>(coords: &mut [P], center: &P, diag: <P::Vect as Vector>::Scalar)
    where P: Point {
    for c in coords.iter_mut() {
        *c = *c * diag + *center.as_vector();
    }
}


/// Computes the convex hull of a set of 3d points.
pub fn convex_hull3<P>(points: &[P]) -> TriMesh<P>
    where P: Point,
          P::Vect: Outer + Mul<<<P as Point>::Vect as Outer>::OuterProductType, Output = <P as Point>::Vect>,
          <P::Vect as Outer>::OuterProductType: EigenQR<<P::Vect as Vector>::Scalar, P::Vect> +
                                                Mul<P, Output = P> +
                                                Add<<P::Vect as Outer>::OuterProductType, Output = <P::Vect as Outer>::OuterProductType> +
                                                Zero + Copy {
    assert!(points.len() != 0, "Cannot compute the convex hull of an empty set of point.");
    assert!(na::dimension::<P>() == 3);

    let mut points = points.to_vec();

    let (norm_center, norm_diag) = normalize(&mut points[..]);

    let mut undecidable_points  = Vec::new();
    let mut horizon_loop_facets = Vec::new();
    let mut horizon_loop_ids    = Vec::new();
    let mut removed_facets      = Vec::new();

    let mut triangles;
    let     denormalizer;

    match get_initial_mesh(&mut points[..], &mut undecidable_points) {
        InitialMesh::Facets(facets, denorm)   => {
            triangles    = facets;
            denormalizer = denorm;
        },
        InitialMesh::ResultMesh(mut mesh) => {
            denormalize(&mut mesh.coords[..], &norm_center, norm_diag);
            return mesh
        }
    }

    let mut i = 0;
    while i != triangles.len() {
        horizon_loop_facets.clear();
        horizon_loop_ids.clear();

        if !triangles[i].valid {
            i = i + 1;
            continue;
        }

        // FIXME: use triangles[i].furthest_point instead.
        let pt_id = support_point(&triangles[i].normal,
                                  &points[..],
                                  &triangles[i].visible_points[..]);

        match pt_id {
            Some(point) => {
                removed_facets.clear();

                triangles[i].valid = false;
                removed_facets.push(i);

                for j in 0usize .. 3 {
                    compute_silhouette(triangles[i].adj[j],
                                       triangles[i].indirect_adj_id[j],
                                       point,
                                       &mut horizon_loop_facets,
                                       &mut horizon_loop_ids,
                                       &points[..],
                                       &mut removed_facets,
                                       &mut triangles[..]);
                }

                if horizon_loop_facets.is_empty() {
                    // Due to inaccuracies, the silhouette could not be computed
                    // (the point seems to be visible from… every triangle).
                    let mut any_valid = false;
                    for j in i + 1 .. triangles.len() {
                        if triangles[j].valid {
                            any_valid = true;
                        }
                    }

                    if any_valid {
                        println!("Warning: exitting an unfinished work.");
                    }

                    // FIXME: this is verry harsh.
                    triangles[i].valid = true;
                    break;
                }

                attach_and_push_facets3(&horizon_loop_facets[..],
                                        &horizon_loop_ids[..],
                                        point,
                                        &points[..],
                                        &mut triangles,
                                        &removed_facets[..],
                                        &mut undecidable_points);
            },
            None => { }
        }

        i = i + 1;
    }

    let mut idx = Vec::new();

    for facet in triangles.iter() {
        if facet.valid {
            idx.push(Point3::new(facet.pts[0] as u32, facet.pts[1] as u32, facet.pts[2] as u32));
        }
    }

    utils::remove_unused_points(&mut points, &mut idx[..]);

    assert!(points.len() != 0, "Internal error: empty output mesh.");

    for point in points.iter_mut() {
        *point = denormalizer * *point;
    }

    denormalize(&mut points[..], &norm_center, norm_diag);

    TriMesh::new(points, None, None, Some(IndexBuffer::Unified(idx)))
}

enum InitialMesh<P: Point, M> {
    Facets(Vec<TriangleFacet<P>>, M),
    ResultMesh(TriMesh<P>)
}

fn build_degenerate_mesh_point<P>(point: P) -> TriMesh<P>
    where P: Point {
    let ta = Point3::new(0u32, 0, 0);
    let tb = Point3::new(0u32, 0, 0);

    TriMesh::new(vec!(point), None, None, Some(IndexBuffer::Unified(vec!(ta, tb))))
}

fn build_degenerate_mesh_segment<P>(dir: &P::Vect, points: &[P]) -> TriMesh<P>
    where P: Point {
    let a = utils::point_cloud_support_point(dir, points);
    let b = utils::point_cloud_support_point(&-*dir, points);

    let ta = Point3::new(0u32, 1, 0);
    let tb = Point3::new(1u32, 0, 0);

    TriMesh::new(vec!(a, b), None, None, Some(IndexBuffer::Unified(vec!(ta, tb))))
}

fn get_initial_mesh<P>(points: &mut [P], undecidable: &mut Vec<usize>) -> InitialMesh<P, <P::Vect as Outer>::OuterProductType>
    where P: Point,
          P::Vect: Outer + Mul<<<P as Point>::Vect as Outer>::OuterProductType, Output = <P as Point>::Vect>,
          <P::Vect as Outer>::OuterProductType: EigenQR<<P::Vect as Vector>::Scalar, P::Vect> +
                                                Add<<P::Vect as Outer>::OuterProductType, Output = <P::Vect as Outer>::OuterProductType> +
                                                Zero + Copy {
    /*
     * Compute the eigenvectors to see if the input datas live on a subspace.
     */
    let cov              = utils::cov(points);
    let (eigvec, eigval) = na::eigen_qr(&cov, &FloatError::epsilon(), 1000);
    let mut eigpairs = [
        (eigvec.column(0), eigval[0]),
        (eigvec.column(1), eigval[1]),
        (eigvec.column(2), eigval[2])
    ];

    /*
     * Sort in deacreasing order wrt. eigenvalues.
     */
    eigpairs.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Less // `Less` and `Greater` are reversed.
        }
        else if a.1 < b.1 {
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    });

    /*
     * Count the dimension the data lives in.
     */
    let mut dimension = 0;
    while dimension < 3 {
        if na::approx_eq_eps(&eigpairs[dimension].1, &na::zero(), &na::cast(1.0e-7f64)) {
            break;
        }

        dimension = dimension + 1;
    }

    match dimension {
        0 => {
            // The hull is a point.
            InitialMesh::ResultMesh(build_degenerate_mesh_point(points[0].clone()))
        },
        1 => {
            // The hull is a segment.
            InitialMesh::ResultMesh(build_degenerate_mesh_segment(&eigpairs[0].0, points))
        },
        2 => {
            // The hull is a triangle.
            // Project into the principal plane…
            let axis1 = &eigpairs[0].0;
            let axis2 = &eigpairs[1].0;

            let mut subspace_points = Vec::with_capacity(points.len());

            for point in points.iter() {
                subspace_points.push(Point2::new(na::dot(point.as_vector(), axis1), na::dot(point.as_vector(), axis2)))
            }

            // … and compute the 2d convex hull.
            let idx = convex_hull2_idx(&subspace_points[..]);

            // Finalize the result, triangulating the polyline.
            let npoints = idx.len();
            let coords  = idx.into_iter().map(|i| points[i].clone()).collect();
            let mut triangles = Vec::with_capacity(npoints + npoints - 4);

            let a = 0u32;

            for id in 1u32 .. npoints as u32 - 1 {
                triangles.push(Point3::new(a, id, id + 1));
                triangles.push(Point3::new(id, a, id + 1));
            }

            InitialMesh::ResultMesh(TriMesh::new(coords, None, None, Some(IndexBuffer::Unified(triangles))))
        },
        3 => {
            // The hull is a polyedra.
            // Find a initial triangle lying on the principal plane…
            let _1: <P::Vect as Vector>::Scalar = na::one();
            let mut diag: P::Vect = na::zero();
            diag[0] = _1 / eigval[0];
            diag[1] = _1 / eigval[1];
            diag[2] = _1 / eigval[2];
            let diag: <P::Vect as Outer>::OuterProductType = Diagonal::from_diagonal(&diag);
            let icov = eigvec * diag * na::transpose(&eigvec);

            for point in points.iter_mut() {
                *point = na::origin::<P>() + icov * *point.as_vector();
            }

            let p1 = support_point_2(&eigpairs[0].0, points).unwrap();
            let p2 = support_point_2(&-eigpairs[0].0, points).unwrap();

            let mut max_area = na::zero();
            let mut p3       = Bounded::max_value();

            for (i, point) in points.iter().enumerate() {
                let area = utils::triangle_area(&points[p1], &points[p2], point);

                if area > max_area {
                    max_area = area ;
                    p3 = i;
                }
            }

            assert!(p3 != Bounded::max_value(), "Internal convex hull error: no triangle found.");

            // Build two facets with opposite normals
            let mut f1 = TriangleFacet::new(p1, p2, p3, points);
            let mut f2 = TriangleFacet::new(p2, p1, p3, points);

            // Link the facets together
            f1.set_facets_adjascency(1, 1, 1, 0, 2, 1);
            f2.set_facets_adjascency(0, 0, 0, 0, 2, 1);

            let mut facets = vec!(f1, f2);

            // … and attribute visible points to each one of them.
            // FIXME: refactor this with the two others.
            let mut ignored = 0usize;
            for point in 0 .. points.len() {
                if point == p1 || point == p2 || point == p3 {
                    continue;
                }

                let mut furthest      = Bounded::max_value();
                let mut furthest_dist = na::zero();

                for (i, curr_facet) in facets.iter().enumerate() {
                    if curr_facet.can_be_seen_by(point, points) {
                        let distance = curr_facet.distance_to_point(point, points);

                        if distance > furthest_dist {
                            furthest      = i;
                            furthest_dist = distance;
                        }
                    }
                }

                if furthest != Bounded::max_value() {
                    facets[furthest].add_visible_point(point, points);
                }
                else {
                    undecidable.push(point);
                    ignored = ignored + 1;
                }

                // If none of the facet can be seen from the point, it is naturally deleted.
            }

            verify_facet_links(0, &facets[..]);
            verify_facet_links(1, &facets[..]);

            InitialMesh::Facets(facets, cov)
        },
        _ => unreachable!()
    }
}

fn support_point<P>(direction: &P::Vect, points : &[P], idx: &[usize]) -> Option<usize>
    where P: Point {
    let mut argmax = None;
    let _max: <P::Vect as Vector>::Scalar = Bounded::max_value();
    let mut max    = -_max;

    for i in idx.iter() {
        let dot = na::dot(direction, points[*i].as_vector());

        if dot > max {
            argmax = Some(*i);
            max    = dot;
        }
    }

    argmax
}

// FIXME: uggly, find a way to refactor all the support point functions!
fn support_point_2<P>(direction: &P::Vect, points : &[P]) -> Option<usize>
    where P: Point {
    let mut argmax = None;
    let _max: <P::Vect as Vector>::Scalar = Bounded::max_value();
    let mut max    = -_max;

    for (id, pt) in points.iter().enumerate() {
        let dot = na::dot(direction, pt.as_vector());

        if dot > max {
            argmax = Some(id);
            max    = dot;
        }
    }

    argmax
}

fn compute_silhouette<P>(facet:          usize,
                         indirect_id :   usize,
                         point:          usize,
                         out_facets:     &mut Vec<usize>,
                         out_adj_idx:    &mut Vec<usize>,
                         points:         &[P],
                         removed_facets: &mut Vec<usize>,
                         triangles:      &mut [TriangleFacet<P>])
    where P: Point {
    if triangles[facet].valid {
        if !triangles[facet].can_be_seen_by_or_is_affinely_dependent_with_contour(point, points, indirect_id) {
            out_facets.push(facet);
            out_adj_idx.push(indirect_id);
        }
        else {
            triangles[facet].valid = false; // The facet must be removed from the convex hull.
            removed_facets.push(facet);

            compute_silhouette(triangles[facet].adj[(indirect_id + 1) % 3],
                               triangles[facet].indirect_adj_id[(indirect_id + 1) % 3],
                               point,
                               out_facets,
                               out_adj_idx,
                               points,
                               removed_facets,
                               triangles);
            compute_silhouette(triangles[facet].adj[(indirect_id + 2) % 3],
                               triangles[facet].indirect_adj_id[(indirect_id + 2) % 3],
                               point,
                               out_facets,
                               out_adj_idx,
                               points,
                               removed_facets,
                               triangles);
        }
    }
}

fn verify_facet_links<P>(ifacet: usize, facets: &[TriangleFacet<P>])
    where P: Point {
    let facet = &facets[ifacet];

    for i in 0usize .. 3 {
        let adji = &facets[facet.adj[i]];

        assert!(
            adji.adj[facet.indirect_adj_id[i]] == ifacet &&
            adji.first_point_from_edge(facet.indirect_adj_id[i]) == facet.second_point_from_edge(adji.indirect_adj_id[facet.indirect_adj_id[i]]) &&
            adji.second_point_from_edge(facet.indirect_adj_id[i]) == facet.first_point_from_edge(adji.indirect_adj_id[facet.indirect_adj_id[i]]))
    }
}

fn attach_and_push_facets3<P>(horizon_loop_facets: &[usize],
                              horizon_loop_ids:    &[usize],
                              point:               usize,
                              points:              &[P],
                              triangles:           &mut Vec<TriangleFacet<P>>,
                              removed_facets:      &[usize],
                              undecidable:         &mut Vec<usize>)
    where P: Point {
    // The horizon is built to be in CCW order.
    let mut new_facets = Vec::with_capacity(horizon_loop_facets.len());

    // Create new facets.
    let mut adj_facet:  usize;
    let mut indirect_id: usize;

    for i in 0 .. horizon_loop_facets.len() {
        adj_facet   = horizon_loop_facets[i];
        indirect_id = horizon_loop_ids[i];

        let facet = TriangleFacet::new(point,
                                       triangles[adj_facet].second_point_from_edge(indirect_id),
                                       triangles[adj_facet].first_point_from_edge(indirect_id),
                                       points);
        new_facets.push(facet);
    }

    // Link the facets together.
    for i in 0 .. horizon_loop_facets.len() {
        let prev_facet;

        if i == 0 {
            prev_facet = triangles.len() + horizon_loop_facets.len() - 1;
        }
        else {
            prev_facet = triangles.len() + i - 1;
        }

        let middle_facet = horizon_loop_facets[i];
        let next_facet   = triangles.len() + (i + 1) % horizon_loop_facets.len();
        let middle_id    = horizon_loop_ids[i];

        new_facets[i].set_facets_adjascency(prev_facet, middle_facet, next_facet,
                                                    2         , middle_id   , 0);
        triangles[middle_facet].adj[middle_id] = triangles.len() + i; // The future id of curr_facet.
        triangles[middle_facet].indirect_adj_id[middle_id] = 1;
    }

    // Assign to each facets some of the points which can see it.
    // FIXME: refactor this with the others.
    for curr_facet in removed_facets.iter() {
        for visible_point in triangles[*curr_facet].visible_points.iter() {
            if *visible_point == point {
                continue;
            }

            let mut furthest      = Bounded::max_value();
            let mut furthest_dist = na::zero();

            for (i, curr_facet) in new_facets.iter_mut().enumerate() {
                if curr_facet.can_be_seen_by(*visible_point, points) {
                    let distance = curr_facet.distance_to_point(*visible_point, points);

                    if distance > furthest_dist {
                        furthest      = i;
                        furthest_dist = distance;
                    }
                }
            }

            if furthest != Bounded::max_value() {
                new_facets[furthest].add_visible_point(*visible_point, points);
            }

            // If none of the facet can be seen from the point, it is naturally deleted.
        }
    }

    // Try to assign collinear points to one of the new facets.
    let mut i = 0;

    while i != undecidable.len() {
        let mut furthest      = Bounded::max_value();
        let mut furthest_dist = na::zero();
        let undecidable_point = undecidable[i];

        for (j, curr_facet) in new_facets.iter_mut().enumerate() {
            if curr_facet.can_be_seen_by(undecidable_point, points) {
                let distance = curr_facet.distance_to_point(undecidable_point, points);

                if distance > furthest_dist {
                    furthest      = j;
                    furthest_dist = distance;
                }
            }
        }

        if furthest != Bounded::max_value() {
            new_facets[furthest].add_visible_point(undecidable_point, points);
            let _ = undecidable.swap_remove(i);
        }
        else {
            i = i + 1;
        }
    }

    // Push facets.
    // FIXME: can we avoid the tmp vector `new_facets` ?
    for curr_facet in new_facets.into_iter() {
        triangles.push(curr_facet);
    }
}


struct TriangleFacet<P: Point> {
    valid:             bool,
    normal:            P::Vect,
    adj:               [usize; 3],
    indirect_adj_id:   [usize; 3],
    pts:               [usize; 3],
    visible_points:    Vec<usize>,
    furthest_point:    usize,
    furthest_distance: <P::Vect as Vector>::Scalar,
    pt_type:           PhantomData<P>
}


impl<P> TriangleFacet<P>
    where P: Point {
    pub fn new(p1: usize, p2: usize, p3: usize, points: &[P]) -> TriangleFacet<P> {
        let p1p2 = points[p2] - points[p1];
        let p1p3 = points[p3] - points[p1];

        let mut normal = utils::cross3(&p1p2, &p1p3);
        if normal.normalize_mut().is_zero() {
            panic!("ConvexHull hull failure: a facet must not be affinely dependent.");
        }

        TriangleFacet {
            valid:             true,
            normal:            normal,
            adj:               [0, 0, 0],
            indirect_adj_id:   [0, 0, 0],
            pts:               [p1, p2, p3],
            visible_points:    Vec::new(),
            furthest_point:    Bounded::max_value(),
            furthest_distance: na::zero(),
            pt_type:           PhantomData
        }
    }

    pub fn add_visible_point(&mut self, pid: usize, points: &[P]) {
        let distance = self.distance_to_point(pid, points);

        if distance > self.furthest_distance {
            self.furthest_distance = distance;
            self.furthest_point    = pid;
        }

        self.visible_points.push(pid);
    }

    pub fn distance_to_point(&self, point: usize, points: &[P]) -> <P::Vect as Vector>::Scalar {
        na::dot(&self.normal, &(points[point] - points[self.pts[0]]))
    }

    pub fn set_facets_adjascency(&mut self,
                                 adj1:   usize,
                                 adj2:   usize,
                                 adj3:   usize,
                                 id_adj1: usize,
                                 id_adj2: usize,
                                 id_adj3: usize) {
        self.indirect_adj_id[0] = id_adj1;
        self.indirect_adj_id[1] = id_adj2;
        self.indirect_adj_id[2] = id_adj3;

        self.adj[0] = adj1;
        self.adj[1] = adj2;
        self.adj[2] = adj3;
    }

    pub fn first_point_from_edge(&self, id: usize) -> usize {
        self.pts[id]
    }

    pub fn second_point_from_edge(&self, id: usize) -> usize {
        self.pts[(id + 1) % 3]
    }

    pub fn can_be_seen_by(&self, point: usize, points: &[P]) -> bool {
        let p0 = &points[self.pts[0]];
        let p1 = &points[self.pts[1]];
        let p2 = &points[self.pts[2]];
        let pt = &points[point];

        let _eps: <P::Vect as Vector>::Scalar = FloatError::epsilon();

        na::dot(&(*pt - *p0), &self.normal) > _eps * na::cast(100.0f64) &&
        !utils::is_affinely_dependent_triangle3(p0, p1, pt) &&
        !utils::is_affinely_dependent_triangle3(p0, p2, pt) &&
        !utils::is_affinely_dependent_triangle3(p1, p2, pt)
    }

    pub fn can_be_seen_by_or_is_affinely_dependent_with_contour(&self,
                                                                point:  usize,
                                                                points: &[P],
                                                                edge:   usize) -> bool {
        let p0 = &points[self.first_point_from_edge(edge)];
        let p1 = &points[self.second_point_from_edge(edge)];
        let pt = &points[point];

        let aff_dep = utils::is_affinely_dependent_triangle3(p0, p1, pt) ||
                      utils::is_affinely_dependent_triangle3(p0, pt, p1) ||
                      utils::is_affinely_dependent_triangle3(p1, p0, pt) ||
                      utils::is_affinely_dependent_triangle3(p1, pt, p0) ||
                      utils::is_affinely_dependent_triangle3(pt, p0, p1) ||
                      utils::is_affinely_dependent_triangle3(pt, p1, p0);

        na::dot(&(*pt - *p0), &self.normal) >= na::zero() || aff_dep
    }
}

/// Computes the convex hull of a set of 2d points.
pub fn convex_hull2<P>(points: &[P]) -> Polyline<P>
    where P: Point {
    assert!(na::dimension::<P>() == 2);

    let idx     = convex_hull2_idx(points);
    let mut pts = Vec::new();

    for id in idx.into_iter() {
        pts.push(points[id].clone());
    }

    Polyline::new(pts, None)
}

/// Computes the convex hull of a set of 2d points and returns only the indices of the hull
/// vertices.
pub fn convex_hull2_idx<P>(points: &[P]) -> Vec<usize>
    where P: Point {
    let mut undecidable_points = Vec::new();
    let mut segments           = get_initial_polyline(points, &mut undecidable_points);

    let mut i = 0;
    while i != segments.len() {
        if !segments[i].valid {
            i = i + 1;
            continue;
        }

        let pt_id = support_point(&segments[i].normal,
                                  points,
                                  &segments[i].visible_points[..]);

        match pt_id {
            Some(point) => {
                segments[i].valid = false;

                attach_and_push_facets2(segments[i].prev,
                                        segments[i].next,
                                        point,
                                        &points[..],
                                        &mut segments,
                                        i,
                                        &mut undecidable_points);
            },
            None => { }
        }

        i = i + 1;
    }

    let mut idx        = Vec::new();
    let mut curr_facet = 0;

    while !segments[curr_facet].valid {
        curr_facet = curr_facet + 1
    }

    let first_facet = curr_facet;

    loop {
        let curr = &segments[curr_facet];

        assert!(curr.valid);

        idx.push(curr.pts[0]);

        curr_facet = curr.next;

        if curr_facet == first_facet {
            break;
        }
    }

    idx
}

fn get_initial_polyline<P>(points: &[P], undecidable: &mut Vec<usize>) -> Vec<SegmentFacet<P>>
    where P: Point {
    let mut res = Vec::new();

    assert!(points.len() >= 2);

    let p1     = support_point_2(&na::canonical_basis_element(0).unwrap(), points).unwrap();
    let mut p2 = p1;

    let direction = [
        -na::canonical_basis_element::<P::Vect>(0).unwrap(),
        na::canonical_basis_element(1).unwrap(),
        -na::canonical_basis_element::<P::Vect>(1).unwrap()
    ];

    for dir in direction.iter() {
        p2 = support_point_2(dir, points).unwrap();

        let p1p2 = points[p2] - points[p1];

        if !na::norm_squared(&p1p2).is_zero() {
            break;
        }
    }

    assert!(p1 != p2, "Failed to build the 2d convex hull of this point cloud.");

    // Build two facets with opposite normals.
    let mut f1 = SegmentFacet::new(p1, p2, 1, 1, points);
    let mut f2 = SegmentFacet::new(p2, p1, 0, 0, points);

    // Attribute points to each facet.
    for i in 0 .. points.len() {
        if i == p1 || i == p2 {
            continue;
        }
        if f1.can_be_seen_by(i, points) {
            f1.visible_points.push(i);
        }
        else if f2.can_be_seen_by(i, points) {
            f2.visible_points.push(i);
        }
        else { // The point is collinear.
            undecidable.push(i);
        }
    }

    res.push(f1);
    res.push(f2);

    res
}

fn attach_and_push_facets2<P>(prev_facet:    usize,
                              next_facet:    usize,
                              point:         usize,
                              points:        &[P],
                              segments:      &mut Vec<SegmentFacet<P>>,
                              removed_facet: usize,
                              undecidable:   &mut Vec<usize>)
    where P: Point {
    let new_facet1_id = segments.len();
    let new_facet2_id = new_facet1_id + 1;
    let prev_pt       = segments[prev_facet].pts[1];
    let next_pt       = segments[next_facet].pts[0];

    let mut new_facet1 = SegmentFacet::new(prev_pt, point, prev_facet, new_facet2_id, points);
    let mut new_facet2 = SegmentFacet::new(point, next_pt, new_facet1_id, next_facet, points);

    segments[prev_facet].next = new_facet1_id;
    segments[next_facet].prev = new_facet2_id;

    // Assign to each facets some of the points which can see it.
    for visible_point in segments[removed_facet].visible_points.iter() {
        if *visible_point == point {
            continue;
        }

        if new_facet1.can_be_seen_by(*visible_point, points) {
            new_facet1.visible_points.push(*visible_point);
        }
        else if new_facet2.can_be_seen_by(*visible_point, points) {
            new_facet2.visible_points.push(*visible_point);
        }
        // If none of the facet can be seen from the point, it is naturally deleted.
    }

    // Try to assign collinear points to one of the new facets
    let mut i = 0;

    while i != undecidable.len() {
        if new_facet1.can_be_seen_by(undecidable[i], points) {
            new_facet1.visible_points.push(undecidable[i]);
            let _ = undecidable.swap_remove(i);
        }
        else if new_facet2.can_be_seen_by(undecidable[i], points) {
            new_facet2.visible_points.push(undecidable[i]);
            let _ = undecidable.swap_remove(i);
        }
        else {
            i = i + 1;
        }
    }

    segments.push(new_facet1);
    segments.push(new_facet2);
}

struct SegmentFacet<P: Point> {
    pub valid:          bool,
    pub normal:         P::Vect,
    pub next:           usize,
    pub prev:           usize,
    pub pts:            [usize; 2],
    pub visible_points: Vec<usize>,
    pt_type:            PhantomData<P>
}

impl<P> SegmentFacet<P>
    where P: Point {
    pub fn new(p1: usize, p2: usize, prev: usize, next: usize, points: &[P]) -> SegmentFacet<P> {
        let p1p2 = points[p2] - points[p1];

        let mut normal = na::zero();

        na::orthonormal_subspace_basis(&p1p2, |e| {
            normal = e;
            false
        });

        if normal.normalize_mut().is_zero() {
            panic!("ConvexHull hull failure: a segment must not be affinely dependent.");
        }

        SegmentFacet {
            valid:          true,
            normal:         normal,
            prev:           prev,
            next:           next,
            pts:            [p1, p2],
            visible_points: Vec::new(),
            pt_type:        PhantomData
        }
    }

    pub fn can_be_seen_by(&self, point: usize, points: &[P]) -> bool {
        let p0 = &points[self.pts[0]];
        let pt = &points[point];

        let _eps: <P::Vect as Vector>::Scalar = FloatError::epsilon();

        na::dot(&(*pt - *p0), &self.normal) > _eps * na::cast(100.0f64)
    }
}



#[cfg(test)]
mod test {
    use na::Point2;
    use procedural;

    #[test]
    fn test_simple_convex_hull2() {
        let points = [
            Point2::new(4.723881f32, 3.597233),
            Point2::new(3.333363,    3.429991),
            Point2::new(3.137215,    2.812263)
            ];

        let chull = super::convex_hull2(points.as_slice());

        assert!(chull.coords.len() == 3);
    }

    #[test]
    fn test_ball_convex_hull() {
        // This trigerred a failure to an affinely dependent facet.
        let sphere = super::sphere(0.4f32, 20, 20, true);
        let points = sphere.coords;
        let chull  = super::convex_hull3(points.as_slice());

        // dummy test, we are just checking that the construction did not fail.
        assert!(chull.coords.len() == chull.coords.len());
    }
}
