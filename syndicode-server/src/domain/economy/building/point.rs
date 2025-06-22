use rstar::{PointDistance, RTreeObject};

use super::model::Building;

// Wrapper struct that rstar can work with.
// It holds a reference to the original building.
pub struct BuildingPoint<'a> {
    pub building: &'a Building,
}

// This tells rstar that the "geometry" of our object is a point.
impl<'a> RTreeObject for BuildingPoint<'a> {
    type Envelope = rstar::AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.building.center.x(), self.building.center.y()])
    }
}

// Implement PointDistance for our wrapper. This is crucial for performance.
// It tells rstar how to calculate the distance from a query point to our object.
// We use squared Euclidean distance because it's faster (avoids sqrt) and
// rstar uses it to find the nearest neighbors efficiently.
impl<'a> PointDistance for BuildingPoint<'a> {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let dx = self.building.center.x() - point[0];
        let dy = self.building.center.y() - point[1];
        dx.powi(2) + dy.powi(2)
    }
}
