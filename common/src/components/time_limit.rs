
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::time::Duration;
use crate::metric_dimension::Second;

#[derive(Debug)]
pub struct Lifespan
{
    pub living_until: Duration,
}

impl Component for Lifespan {
    type Storage = DenseVecStorage<Self>;
}

impl Lifespan
{
    pub fn new(secs_to_live: Second, timing_start: Duration) -> Self {
        Lifespan {
            living_until: timing_start + secs_to_live.into_duration(),
        }
    }
}
