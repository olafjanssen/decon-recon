extends Control

var description_container: VBoxContainer
var card_container: HBoxContainer

var aspect_card = preload("res://scenes/cards/aspect_card.tscn")
var description_button = preload("res://scenes/game_modes/modality_con/aspect_description_button.tscn")

func _ready() -> void:

	var campaign_data = CampaignData.new()
	add_child(campaign_data)

	# find an aspect card
	var modalities = campaign_data.get_modalities()
	var modality = modalities.pick_random()
	var used_aspects = campaign_data.get_modality_aspects(modality.id)
	var used_aspect = used_aspects.pick_random()

	## set the aspect cards
	card_container = get_node("HBoxContainer")
	for n in card_container.get_children():
		card_container.remove_child(n)
		n.queue_free()
		
	var card = aspect_card.instantiate()
	card.set_aspect(used_aspect)
	card_container.add_child(card)

	
	## set the aspect cards
	description_container = get_node("VBoxContainer")
	for n in description_container.get_children():
		description_container.remove_child(n)
		n.queue_free()

	var aspects = []
	aspects.append(used_aspect)

	while len(aspects) < 3:
		var new_aspect = used_aspects.pick_random()
		if !aspects.has(new_aspect):
			aspects.append(new_aspect)
#
	aspects.shuffle()
#
	for aspect in aspects:
		var description = description_button.instantiate()
		description.set_aspect(aspect)
		description_container.add_child(description)
