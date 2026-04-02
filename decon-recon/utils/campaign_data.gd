extends Node

class_name CampaignData

var campaign_data = {}
var current_campaign_id = ""

func _ready():
	# Load all campaigns from the campaigns directory
	load_all_campaigns()

func load_all_campaigns():
	var dir = DirAccess.open("res://campaigns")
	if dir:
		dir.list_dir_begin()
		var file_name = dir.get_next()
		while file_name != "":
			if file_name.ends_with(".json"):
				var campaign_id = file_name.replace(".json", "")
				var path = "res://campaigns/" + file_name
				var file = FileAccess.open(path, FileAccess.READ)
				if file:
					var content = file.get_as_text()
					var json = JSON.new()
					var error = json.parse(content)
					if error == OK:
						campaign_data[campaign_id] = json.data
						if current_campaign_id == "" or campaign_id == "small":
							current_campaign_id = campaign_id
					file.close()
			file_name = dir.get_next()
		dir.list_dir_end()
	else:
		push_error("Failed to open campaigns directory")

func set_current_campaign(campaign_id: String) -> bool:
	if campaign_data.has(campaign_id):
		current_campaign_id = campaign_id
		return true
	return false

func get_current_campaign_id() -> String:
	return current_campaign_id

func get_campaign_ids() -> Array:
	return campaign_data.keys()

# Campaign metadata
func get_campaign_title(campaign_id: String = "") -> String:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("campaign", {}).get("title", "Unknown Campaign")

func get_campaign_description(campaign_id: String = "") -> String:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("campaign", {}).get("description", "")

func get_campaign_introduction(campaign_id: String = "") -> String:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("campaign", {}).get("introduction", "")

func get_campaign_resolution(campaign_id: String = "") -> String:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("campaign", {}).get("resolution", "")

func get_campaign_hub_location(campaign_id: String = "") -> String:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("campaign", {}).get("hub_location", "")

# Characters
func get_characters(campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("characters", [])

func get_character_by_id(character_id: String, campaign_id: String = "") -> Dictionary:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	for character in campaign_data.get(id, {}).get("characters", []):
		if character.get("id", "") == character_id:
			return character
	return {}

func get_character_name(character_id: String, campaign_id: String = "") -> String:
	return get_character_by_id(character_id, campaign_id).get("name", "Unknown")

func get_character_description(character_id: String, campaign_id: String = "") -> String:
	return get_character_by_id(character_id, campaign_id).get("description", "")

func get_character_location(character_id: String, campaign_id: String = "") -> String:
	return get_character_by_id(character_id, campaign_id).get("location", "")

func get_character_secret(character_id: String, campaign_id: String = "") -> String:
	return get_character_by_id(character_id, campaign_id).get("secret", "")

func get_character_modalities(character_id: String, campaign_id: String = "") -> Array:
	return get_character_by_id(character_id, campaign_id).get("preferential_modalities", [])

# Modalities
func get_modalities(campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("modalities", [])

func get_modality_by_id(modality_id: String, campaign_id: String = "") -> Dictionary:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	for modality in campaign_data.get(id, {}).get("modalities", []):
		if modality.get("id", "") == modality_id:
			return modality
	return {}

func get_modality_name(modality_id: String, campaign_id: String = "") -> String:
	return get_modality_by_id(modality_id, campaign_id).get("name", "Unknown")

func get_modality_aspects(modality_id: String, campaign_id: String = "") -> Array:
	return get_modality_by_id(modality_id, campaign_id).get("aspects", [])

func get_aspect_by_id(aspect_id: String, campaign_id: String = "") -> Dictionary:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	for modality in campaign_data.get(id, {}).get("modalities", []):
		for aspect in modality.get("aspects", []):
			if aspect.get("id", "") == aspect_id:
				return aspect
	return {}

# Substants (facts)
func get_substants(campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("substants", [])

func get_substant_by_id(substant_id: String, campaign_id: String = "") -> Dictionary:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	for substant in campaign_data.get(id, {}).get("substants", []):
		if substant.get("id", "") == substant_id:
			return substant
	return {}

func get_substant_factoid(substant_id: String, campaign_id: String = "") -> String:
	return get_substant_by_id(substant_id, campaign_id).get("factoid", "")

# Utterances
func get_utterances(campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	return campaign_data.get(id, {}).get("utterances", [])

func get_utterances_by_character(character_id: String, campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	var result = []
	for utterance in campaign_data.get(id, {}).get("utterances", []):
		if utterance.get("character_id", "") == character_id:
			result.append(utterance)
	return result

func get_utterance_by_id(utterance_id: String, campaign_id: String = "") -> Dictionary:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	for utterance in campaign_data.get(id, {}).get("utterances", []):
		if utterance.get("id", "") == utterance_id:
			return utterance
	return {}

# Helper functions
func get_utterances_by_substant(substant_id: String, campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	var result = []
	for utterance in campaign_data.get(id, {}).get("utterances", []):
		if utterance.get("substant_id", "") == substant_id:
			result.append(utterance)
	return result

func get_utterances_by_aspect(aspect_id: String, campaign_id: String = "") -> Array:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	var result = []
	for utterance in campaign_data.get(id, {}).get("utterances", []):
		if utterance.get("used_aspect", "") == aspect_id:
			result.append(utterance)
	return result

func get_character_utterances_with_construction(character_id: String, campaign_id: String = "") -> Dictionary:
	var id = campaign_id if campaign_id != "" else current_campaign_id
	var result = {}
	for utterance in campaign_data.get(id, {}).get("utterances", []):
		if utterance.get("character_id", "") == character_id:
			var constructed_from = utterance.get("constructed_from", null)
			if constructed_from:
				if not result.has(constructed_from):
					result[constructed_from] = []
				result[constructed_from].append(utterance)
	return result
