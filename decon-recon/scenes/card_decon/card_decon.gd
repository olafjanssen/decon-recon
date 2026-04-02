extends Control

var text_container: RichTextLabel
var card_container: HBoxContainer

var aspect_card = preload("res://scenes/card_decon/aspect_card.tscn")

func _ready() -> void:
	print("Hello!")
	var campaign_data = CampaignData.new()
	add_child(campaign_data)

	# find an utterance
	var utterances = campaign_data.get_utterances()

	var utterance = utterances.pick_random()

	# set the utterance
	text_container = get_node("VBoxContainer/Utterance/RichTextLabel")
	text_container.text = utterance.utterance

	# set the aspect cards
	card_container = get_node("VBoxContainer/CardsContainer/HBoxContainer")
	for n in card_container.get_children():
		card_container.remove_child(n)
		n.queue_free()

	var used_aspect = campaign_data.get_aspect_by_id(utterance.used_aspect)
	var modality = used_aspect.id.split(':')[0]
	var modality_aspects = campaign_data.get_modality_aspects(modality)

	var aspects = []
	aspects.append(used_aspect)
	
	while len(aspects) < 3:
		var new_aspect = modality_aspects.pick_random()
		if !aspects.has(new_aspect): 
			aspects.append(new_aspect)
	
	aspects.shuffle()	

	for aspect in aspects:
		print(aspect)
		var card = aspect_card.instantiate()
		card.set_aspect(aspect)
		card_container.add_child(card)
