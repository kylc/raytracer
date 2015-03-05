use std::num::Float;
use ray::Ray;
use intersection::Intersection;
use object::Object;

pub struct Scene<'a> {
    pub objects: Vec<Object<'a>>
}

impl<'a> Scene<'a> {
    pub fn intersects(&'a self, ray: &Ray) -> Option<(Intersection, &'a Object)> {
        // Check if this ray intersects any objects in the scene.
        // TODO: This could perhaps be done easier with a filter_map and a
        // sort_by.
        self.objects.iter().fold(None, |a, ref b| {
            let prev_distance = match a {
                Some((ref intersection, _)) => intersection.distance,
                None => Float::infinity()

            };

            match b.surface.intersects(&ray) {
                Some(ref intersection) if intersection.distance < prev_distance => {
                    Some((*intersection, b))
                },
                _   => a
            }
        })
    }
}
