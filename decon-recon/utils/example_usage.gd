# Example of how to use CampaignData in other nodes
# This script shows how to access campaign data from any node in your game

# Method 1: Access via autoload (recommended)
# Add this to your Project Settings -> AutoLoad:
# Script: res://decon-recon/campaign_data.gd
# Node Name: CampaignData

# Then you can access it from anywhere like this:
# var title = CampaignData.get_campaign_title()
# var characters = CampaignData.get_characters()

# Method 2: Add as a child node
# extends SomeNode
#
# func _ready():
#     var campaign_data = CampaignData.new()
#     add_child(campaign_data)
#     await campaign_data.ready
#
#     # Now you can use it
#     var intro = campaign_data.get_campaign_introduction()
#     print(intro)

# Method 3: Get from scene tree (if already loaded)
# func _ready():
#     var campaign_data = get_node("/root/CampaignData")
#     if campaign_data:
#         var resolution = campaign_data.get_campaign_resolution()
#         print(resolution)

# ===== Common Usage Examples =====

# Get campaign metadata
# var title = CampaignData.get_campaign_title()
# var description = CampaignData.get_campaign_description()
# var hub_location = CampaignData.get_campaign_hub_location()

# Get all characters in current campaign
# var characters = CampaignData.get_characters()
# for character in characters:
#     print(character.name, "is at", character.location)

# Get specific character data
# var elias = CampaignData.get_character_by_id("elias")
# var elias_name = CampaignData.get_character_name("elias")
# var elias_secret = CampaignData.get_character_secret("elias")
# var elias_modalities = CampaignData.get_character_modalities("elias")

# Get all utterances for a character
# var elias_utterances = CampaignData.get_utterances_by_character("elias")
# for utterance in elias_utterances:
#     print(utterance.utterance)

# Get substants (facts)
# var all_facts = CampaignData.get_substants()
# var specific_fact = CampaignData.get_substant_by_id("S1")
# var fact_text = CampaignData.get_substant_factoid("S1")

# Get modalities and aspects
# var all_modalities = CampaignData.get_modalities()
# var core_modality = CampaignData.get_modality_by_id("core")
# var facts_aspect = CampaignData.get_aspect_by_id("core:facts")

# Switch campaigns
# var success = CampaignData.set_current_campaign("town_hall_murder")
# if success:
#     print("Switched to:", CampaignData.get_campaign_title())

# Get data from specific campaign (without switching)
# var title = CampaignData.get_campaign_title("town_hall_murder")
# var chars = CampaignData.get_characters("town_hall_murder")

# ===== Advanced Queries =====

# Get all utterances related to a specific fact
# var utterances_about_S1 = CampaignData.get_utterances_by_substant("S1")

# Get all utterances using a specific aspect
# var warnings = CampaignData.get_utterances_by_aspect("pragmatic:warnings")

# Get character utterances organized by construction hierarchy
# var construction_tree = CampaignData.get_character_utterances_with_construction("elias")
