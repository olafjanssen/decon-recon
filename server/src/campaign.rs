use crate::models::{Campaign, CampaignData, Character, ModalityLayer, Substant};
use std::path::Path;
use std::fs;
use std::io;
use toml;

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
