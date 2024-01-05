mod input_playback;
pub use input_playback::PlaybackTestGear;

use bevy::{ecs::system::SystemId, prelude::*};

#[derive(Resource, Debug)]
pub struct AssertSystem(pub SystemId);
