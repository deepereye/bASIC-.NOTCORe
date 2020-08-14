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
        