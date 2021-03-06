use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::math::Vector2;
use crate::metric_dimension::MeterPerSec;

#[derive(Debug)]
pub struct Velocity(pub Vector2<MeterPerSec>);

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity(Vector2::new(MeterPerSec(0.0), MeterPerSec(0.0)))
    }
}
