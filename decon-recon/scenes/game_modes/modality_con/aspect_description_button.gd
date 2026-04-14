extends PanelContainer

var aspect_data

func _ready():
	# Connect the button's pressed signal
	var button = get_node("Button")
	button.connect("pressed", Callable(self, "_on_button_pressed"))

func set_aspect(aspect) -> void:
	var description_label = get_node("MarginContainer/RichTextLabel")	
	description_label.text = aspect.description
	aspect_data = aspect

func _on_button_pressed():
	# Emit signal with the aspect data when button is pressed
	emit_signal("description_clicked", aspect_data)

signal description_clicked(selected_aspect)
