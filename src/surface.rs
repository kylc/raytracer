use intersection::Intersection;
use na::{Point3, Vector3};
use ray::Ray;

pub trait Surface {
    // Check if a ray intersects with the surface.
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
    fn normal_towards(&self, point: Point3<f32>) -> Vector3<f32>;
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

impl Surface for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Equations taken from http://en.wikipedia.org/wiki/Line–sphere_intersection
        let oc = ray.origin - self.center;

        // Find quadratic equation coefficients.
        let a = ray.direction.norm_squared();
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.norm_squared() - self.radius.powi(2);

        let det2 = b.powi(2) - 4.0 * a * c;

        // Three options based on the value of det^2:
        // 1) det^2 < 0.0: no solutions
        // 2) det^2 = 0.0: one solution
        // 3) det^2 > 0.0: two solutions (pick the closest)
        if det2 >= 0.0 {
            // Only sqrt det once we know we have to.
            let det = det2.sqrt();

            // Calculate the two solutions to the quadratic equation.
            let t1 = (-b + det) / (2.0 * a);
            let t2 = (-b - det) / (2.0 * a);

            // Choose the positive intersection with the minimum distance.
            let dist = if t1 > 0.0 && t1 < t2 {
                Some(t1)
            } else if t2 > 0.0 && t2 < t1 {
                Some(t2)
            } else {
                None
            };

            dist.map(|d| Intersection::new_from_distance(d, ray, self))
        } else {
            None
        }
    }

    fn normal_towards(&self, point: Point3<f32>) -> Vector3<f32> {
        // The normal is a ray traced from the center of the sphere to the given
        // point, normalized.
        (point - self.center).normalize()
    }
}

pub struct Plane {
    pub normal: Vector3<f32>,
    pub offset: f32,
}

impl Surface for Plane {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let origin = ray.origin.coords.map(|x| x - self.offset);

        let d = self.normal.dot(&ray.direction);
        if d == 0.0 {
            return None;
        };
        let t = -self.normal.dot(&origin) / d;

        if t > 0.0 {
            Some(Intersection::new_from_distance(t, &ray, self))
        } else {
            None
        }
    }

    fn normal_towards(&self, _point: Point3<f32>) -> Vector3<f32> {
        self.normal
    }
}

pub struct Triangle {
    pub vertices: [Point3<f32>; 3],
}

impl Surface for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // Per http://www.cs.virginia.edu/~gfx/Courses/2003/ImageSynthesis/papers/Acceleration/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let pvec = ray.direction.cross(&e2);

        let det = pvec.dot(&e1);
        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.vertices[0];
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&e1);
        let v = ray.direction.dot(&qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = e2.dot(&qvec) * inv_det;
        if t > 0.0 {
            Some(Intersection::new_from_distance(t, &ray, self))
        } else {
            None
        }
    }

    fn normal_towards(&self, point: Point3<f32>) -> Vector3<f32> {
        let u = self.vertices[1] - self.vertices[0];
        let v = self.vertices[2] - self.vertices[0];

        let normal = u.cross(&v).normalize();

        // The triangle is two-sided, so we need to figure out which normal we
        // should return.
        if normal.dot(&point.coords) > 0.0 {
            normal
        } else {
            -normal
        }
    }
}
