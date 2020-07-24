# This .gd file is not used, it just serves as a comparison with the godot-rust implementation.

extends Node

@export var mob_scene: PackedScene
var score

func _ready():
	randomize()


func game_over():
	$ScoreTimer.stop()
	$MobTimer.stop()
	$Hud.show_game_over()
	$Music.stop()
	$DeathSound.play()


func new_game():
	get_tree().call_group("mobs", "queue_free"