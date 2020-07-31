# This .gd file is not used, it just serves as a comparison with the godot-rust implementation.

extends RigidBody2D

func _ready():
	$AnimatedSprite2D.playing = true
	var mob_types = $An