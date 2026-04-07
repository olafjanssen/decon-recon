extends PanelContainer

func set_aspect(aspect) -> void:
	var description_label = get_node("MarginContainer/RichTextLabel")	
	description_label.text = aspect.description
