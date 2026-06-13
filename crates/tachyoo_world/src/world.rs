use crate::config::WorldConfig;
use bevy_ecs::world::World as EcsWorld;

use bevy_ecs::schedule::ScheduleLabel;

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct Tick;

pub struct World {
    ecs_world: EcsWorld,
    
}

impl World {
    fn new(config: WorldConfig) -> World {
        let ecs_world=EcsWorld::new();
        
        World { ecs_world }
    }
}