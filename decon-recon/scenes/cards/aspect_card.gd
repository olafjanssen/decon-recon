extends Control

func set_aspect(aspect) -> void:
	var name_label = get_node("PanelContainer/MarginContainer/VBoxContainer/Name")
	var layman_label = get_node("PanelContainer/MarginContainer/VBoxContainer/Layman")
	var symbol_label = get_node("PanelContainer/MarginContainer/VBoxContainer/Symbol")
	var description_label = get_node("PanelContainer/MarginContainer/VBoxContainer/Description")

	name_label.text = aspect.name
	layman_label.text = aspect.layman_name
	symbol_label.text = aspect.icon
	description_label.text = aspect.description
