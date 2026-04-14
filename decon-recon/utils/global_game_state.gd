extends Node

class_name GlobalGameState

var energy = 150

func _ready() -> void:
	Signals.connect("energy_spent", _on_energy_spent)


func _on_energy_spent(energy_spent):
	energy -= energy_spent
