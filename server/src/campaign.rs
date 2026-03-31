use crate::models::{
    Campaign, CampaignData, Character, ModalityLayer, Substant, Utterance, UtterancesWrapper,
};
use std::fs;
use std::io;
use std::path::Path;
use toml;
use uuid::Uuid;

#[derive(Debug, serde::Deserialize)]
struct CharactersWrapper {
    characters: Vec<Character>,
}

#[derive(Debug, serde::Deserialize)]
struct ModalitiesWrapper {
    layers: Vec<ModalityLayer>,
}

#[derive(Debug, serde::Deserialize)]
struct SubstantsWrapper {
    substants: Vec<Substant>,
}

pub fn load_campaign(campaign_id: &str, data_path: &str) -> Result<CampaignData, io::Error> {
    let campaign_path = Path::new(data_path).join(campaign_id);

    // Load campaign.toml
    let campaign_content = fs::read_to_string(campaign_path.join("campaign.toml"))?;
    let campaign: Campaign = toml::from_str(&campaign_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    // Load characters.toml
    let characters_content = fs::read_to_string(campaign_path.join("characters.toml"))?;
    let wrapper: CharactersWrapper = toml::from_str(&characters_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    let characters = wrapper.characters;

    // Load modalities.toml
    let modalities_content = fs::read_to_string(campaign_path.join("modalities.toml"))?;
    let wrapper: ModalitiesWrapper = toml::from_str(&modalities_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    let modalities = wrapper.layers;

    // Load substants.toml
    let substants_content = fs::read_to_string(campaign_path.join("substants.toml"))?;
    let wrapper: SubstantsWrapper = toml::from_str(&substants_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    let substants = wrapper.substants;

    Ok(CampaignData {
        campaign,
        characters,
        modalities,
        substants,
    })
}

pub fn list_campaigns(data_path: &str) -> Result<Vec<String>, io::Error> {
    let mut campaigns = Vec::new();
    let path = Path::new(data_path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().join("campaign.toml").exists() {
                if let Some(name) = entry.file_name().to_str() {
                    campaigns.push(name.to_string());
                }
            }
        }
    }

    Ok(campaigns)
}

pub fn campaign_exists(campaign_id: &str, data_path: &str) -> bool {
    let campaign_path = Path::new(data_path).join(campaign_id).join("campaign.toml");
    campaign_path.exists()
}

pub fn get_modalities_with_levels(
    campaign_data: &CampaignData,
    aspect_ids: &[&str],
) -> Vec<crate::models::ModalityAspectWithLevel> {
    let mut modalities_with_levels = Vec::new();

    for aspect_id in aspect_ids {
        if let Some(aspect) = campaign_data.find_modality_aspect(aspect_id) {
            // Extract level from aspect ID (format: "layer:aspect")
            let level = aspect_id
                .split(':')
                .next()
                .and_then(|layer_id| {
                    campaign_data
                        .modalities
                        .iter()
                        .find(|layer| layer.id == layer_id)
                        .map(|layer| layer.level)
                })
                .unwrap_or(0);

            modalities_with_levels.push(crate::models::ModalityAspectWithLevel {
                aspect: aspect.clone(),
                level,
            });
        }
    }

    // Sort by level (low to high)
    modalities_with_levels.sort_by_key(|m| m.level);

    modalities_with_levels
}

pub fn load_utterances(campaign_id: &str, data_path: &str) -> Result<Vec<Utterance>, io::Error> {
    let utterances_path = Path::new(data_path)
        .join(campaign_id)
        .join("utterances.toml");

    if !utterances_path.exists() {
        return Ok(Vec::new());
    }

    let utterances_content = fs::read_to_string(utterances_path)?;
    let wrapper: UtterancesWrapper = toml::from_str(&utterances_content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    Ok(wrapper.utterances)
}

pub fn save_utterance(
    campaign_id: &str,
    data_path: &str,
    utterance: &Utterance,
) -> Result<(), io::Error> {
    let utterances_path = Path::new(data_path)
        .join(campaign_id)
        .join("utterances.toml");

    // Load existing utterances
    let mut utterances = load_utterances(campaign_id, data_path)?;

    // Add new utterance
    utterances.push(utterance.clone());

    // Create wrapper and save
    let wrapper = UtterancesWrapper { utterances };
    let toml_content = toml::to_string(&wrapper)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    fs::write(utterances_path, toml_content)?;

    Ok(())
}

pub fn create_utterance(
    character_id: &str,
    substant_id: &str,
    utterance_text: &str,
    constructed_from: Option<&str>,
    used_aspect: Option<&str>,
) -> Utterance {
    Utterance {
        id: Uuid::new_v4().to_string(),
        character_id: character_id.to_string(),
        substant_id: substant_id.to_string(),
        utterance: utterance_text.to_string(),
        insight: None,
        constructed_from: constructed_from.map(|s| s.to_string()),
        used_aspect: used_aspect.map(|s| s.to_string()),
    }
}

pub fn create_utterance_with_insight(
    character_id: &str,
    substant_id: &str,
    utterance_text: &str,
    insight: Option<&str>,
    constructed_from: Option<&str>,
    used_aspect: Option<&str>,
) -> Utterance {
    Utterance {
        id: Uuid::new_v4().to_string(),
        character_id: character_id.to_string(),
        substant_id: substant_id.to_string(),
        utterance: utterance_text.to_string(),
        insight: insight.map(|s| s.to_string()),
        constructed_from: constructed_from.map(|s| s.to_string()),
        used_aspect: used_aspect.map(|s| s.to_string()),
    }
}
