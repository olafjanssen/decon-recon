extends Control

var description_container: VBoxContainer
var card_container: HBoxContainer
var campaign_data
var game_parameters
var current_aspect

var aspect_card = preload("res://scenes/cards/aspect_card.tscn")
var description_button = preload("res://scenes/game_modes/modality_con/aspect_description_button.tscn")


var game_mode
var game_start
var game_end
var play_button
var play_again_button
var rounds_left

signal description_selected(correct: bool, selected_aspect)

func _ready() -> void:
	campaign_data = CampaignData.new()
	add_child(campaign_data)
	
	game_parameters = GameParameters.new().getModalityConParameters()
	
	play_button = get_node("GameStart/Button")
	play_button.connect("pressed", Callable(self, "_on_play_pressed"))
	play_again_button = get_node("GameEnd/PanelContainer/Button")
	play_again_button.connect("pressed", Callable(self, "_on_play_pressed"))
	
	game_mode = get_node("GameMode")
	game_start = get_node("GameStart")
	game_end = get_node("GameEnd")
	
	# Start setup
	game_mode.visible = false
	game_end.visible = false
	game_start.visible = true

	play_button.text = "Play (" + str(game_parameters.energy_cost) + "⚡️)" 

func show_next_task():
	print("Showing next task")
	# find an aspect card
	var modalities = campaign_data.get_modalities()
	var modality = modalities.pick_random()
	var used_aspects = campaign_data.get_modality_aspects(modality.id)
	var used_aspect = used_aspects.pick_random()

	## set the aspect cards
	card_container = get_node("GameMode/HBoxContainer")
	for n in card_container.get_children():
		card_container.remove_child(n)
		n.queue_free()

	var card = aspect_card.instantiate()
	card.set_aspect(used_aspect)
	card_container.add_child(card)

	# Store the current aspect for comparison
	current_aspect = used_aspect

	## set the aspect cards
	description_container = get_node("GameMode/VBoxContainer")
	for n in description_container.get_children():
		description_container.remove_child(n)
		n.queue_free()

	var aspects = []
	aspects.append(used_aspect)

	var trials = 0
	while len(aspects) < 3 and trials < 100:
		trials = trials + 1
		var new_aspect = used_aspects.pick_random()
		if !aspects.has(new_aspect):
			aspects.append(new_aspect)

	aspects.shuffle()

	for aspect in aspects:
		var description = description_button.instantiate()
		description.set_aspect(aspect)
		description.connect("description_clicked", Callable(self, "_on_description_clicked"))
		description_container.add_child(description)	

func _on_description_clicked(selected_aspect):
	print("Description clicked!")
	var is_correct = selected_aspect == current_aspect
	emit_signal("description_selected", is_correct, selected_aspect)
	
	rounds_left -= 1
	if rounds_left>0:
		show_next_task()
	else:
		game_mode.visible = false
		game_end.visible = true
		game_start.visible = false
		
		game_parameters = GameParameters.new().getModalityConParameters()
		play_again_button.text = "Play Again (" + str(game_parameters.energy_cost) + "⚡️)" 
	

func _on_play_pressed():
	game_mode.visible = true
	game_end.visible = false
	game_start.visible = false
	
	rounds_left = game_parameters.rounds
	show_next_task()
