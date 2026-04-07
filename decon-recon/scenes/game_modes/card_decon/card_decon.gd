extends Control

var text_container: RichTextLabel
var card_container: HBoxContainer

var aspect_card = preload("res://scenes/cards/aspect_card.tscn")

func _ready() -> void:
	print("Hello!")
	var campaign_data = CampaignData.new()
	add_child(campaign_data)

	# find an utterance
	var utterances = campaign_data.get_utterances()

	# Find an utterance that has a constructed_from field
	var utterance = utterances.pick_random()
	while (utterance.construction_depth<1):
		utterance = utterances.pick_random()

	var source_utterance = campaign_data.get_utterance_by_id(utterance.constructed_from)

	# set the utterance
	text_container = get_node("VBoxContainer/Utterance/MarginContainer/RichTextLabel")
	text_container.text = utterance.utterance

	# show diff
	var diff_utility = DiffUtility.new()

	# Show only insertions (new text), hide deletions (old text)
	var diff_result = diff_utility.calculate_diff(source_utterance.utterance, utterance.utterance, false, true)
	diff_result = diff_result.replace("[b]","[bgcolor=purple]").replace("[/b]","[/bgcolor]")

	text_container.bbcode_text = diff_result
	#text_container.bbcode_text = utterance.utterance


	# set the aspect cards
	card_container = get_node("HBoxContainer")
	for n in card_container.get_children():
		card_container.remove_child(n)
		n.queue_free()

	var used_aspect = campaign_data.get_aspect_by_id(utterance.used_aspect)
	var modality = used_aspect.id.split(':')[0]
	var modality_aspects = campaign_data.get_modality_aspects(modality)

	var aspects = []
	aspects.append(used_aspect)
	print(used_aspect)
	
	while len(aspects) < 3:
		var new_aspect = modality_aspects.pick_random()
		if !aspects.has(new_aspect):
			aspects.append(new_aspect)

	aspects.shuffle()

	for aspect in aspects:
		var card = aspect_card.instantiate()
		card.set_aspect(aspect)
		card_container.add_child(card)
