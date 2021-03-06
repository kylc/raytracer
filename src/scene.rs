use intersection::Intersection;
use object::Object;
use ray::Ray;
use std::f32;

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn intersects(&self, ray: &Ray) -> Option<(Intersection, &Object)> {
        // Check if this ray intersects any objects in the scene.
        // TODO: This could perhaps be done easier with a filter_map and a
        // sort_by.
        self.objects.iter().fold(None, |a, ref b| {
            let prev_distance = match a {
                Some((ref intersection, _)) => intersection.distance,
                None => f32::INFINITY,
            };

            match b.surface.intersects(&ray) {
                Some(ref intersection) if intersection.distance < prev_distance => {
                    Some((*intersection, b))
                }
                _ => a,
            }
        })
    }
}
