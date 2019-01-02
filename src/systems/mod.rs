mod physics_stepper;
mod sync_bodies_from_physics;
mod sync_bodies_to_physics;
mod sync_colliders_to_physics;
mod sync_gravity_to_physics;

use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::core::specs::DispatcherBuilder;

use crate::time_step::TimeStep;

pub use self::physics_stepper::PhysicsStepperSystem;
pub use self::sync_bodies_from_physics::*;
pub use self::sync_bodies_to_physics::SyncBodiesToPhysicsSystem;
pub use self::sync_colliders_to_physics::SyncCollidersToPhysicsSystem;
pub use self::sync_gravity_to_physics::SyncGravityToPhysicsSystem;

pub const SYNC_BODIES_TO_PHYSICS_SYSTEM: &str = "sync_bodies_to_physics_system";
pub const SYNC_GRAVITY_TO_PHYSICS_SYSTEM: &str = "sync_gravity_to_physics_system";
pub const SYNC_COLLIDERS_TO_PHYSICS_SYSTEM: &str = "sync_colliders_to_physics_system";
pub const PHYSICS_STEPPER_SYSTEM: &str = "physics_stepper_system";
pub const SYNC_BODIES_FROM_PHYSICS_SYSTEM: &str = "sync_bodies_from_physics_system";

pub struct PhysicsBundle<'a> {
    dep: &'a [&'a str],
    timestep: TimeStep,
    max_timesteps: i32,
}

impl Default for PhysicsBundle<'_> {
    fn default() -> Self {
        Self {
            dep: Default::default(),
            timestep: Default::default(),
            max_timesteps: 10,
        }
    }
}

impl<'a> PhysicsBundle<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_dep(mut self, dep: &'a [&'a str]) -> Self {
        self.dep = dep;
        self
    }

    /// Set the timestep to use for the `PhysicsStepperSystem`
    pub fn with_timestep(mut self, timestep: TimeStep) -> Self {
        self.timestep = timestep;
        self
    }

    /// Set the maximum number of physics timesteps to simulate in a single run of the `PhysicsStepperSystem`
    pub fn with_max_timesteps(mut self, max_timesteps: i32) -> Self {
        self.max_timesteps = max_timesteps;
        self
    }
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for PhysicsBundle<'c> {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            SyncBodiesToPhysicsSystem::new(),
            SYNC_BODIES_TO_PHYSICS_SYSTEM,
            self.dep,
        );
        builder.add(
            SyncGravityToPhysicsSystem::new(),
            SYNC_GRAVITY_TO_PHYSICS_SYSTEM,
            self.dep,
        );

        builder.add(
            SyncCollidersToPhysicsSystem::new(),
            SYNC_COLLIDERS_TO_PHYSICS_SYSTEM,
            &[SYNC_BODIES_TO_PHYSICS_SYSTEM],
        );

        builder.add(
            PhysicsStepperSystem::new(self.timestep, self.max_timesteps),
            PHYSICS_STEPPER_SYSTEM,
            &[
                SYNC_BODIES_TO_PHYSICS_SYSTEM,
                SYNC_GRAVITY_TO_PHYSICS_SYSTEM,
                SYNC_COLLIDERS_TO_PHYSICS_SYSTEM,
            ],
        );

        builder.add(
            SyncBodiesFromPhysicsSystem::new(),
            SYNC_BODIES_FROM_PHYSICS_SYSTEM,
            &[PHYSICS_STEPPER_SYSTEM],
        );

        Ok(())
    }
}
