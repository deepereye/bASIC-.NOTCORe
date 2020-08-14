use godot::engine::{AnimatedSprite2D, RigidBody2D};
use godot::prelude::*;
use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
enum MobType {
    Walk,
    Swim,
    Fly,
}

impl MobType {
    fn to_str(self) -> GodotString {
        match self {
            MobType::Walk => "walk".into(),
            MobType::Swim => "swim".into(),
            MobType::Fly => "fly".into(),
        }
    }
}

const MOB_TYPES: [MobType; 3] = [MobType::Walk, MobType::Swim, MobType::Fly];

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,

    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func