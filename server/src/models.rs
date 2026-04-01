use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Campaign {
    pub id: String,
    pub title: String,
    pub hub_location: String,
    pub description: String,
    pub introduction: String,
    pub resolution: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub location: String,
    pub description: String,
    pub preferential_modalities: Vec<String>,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModalityLayer {
    pub level: u32,
    pub id: String,
    pub name: String,
    pub layman_name: String,
    pub full_description: String,
    pub aspects: Vec<ModalityAspect>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModalityAspect {
    pub id: String,
    pub name: String,
    pub layman_name: String,
    pub description: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModalityAspectWithLevel {
    pub aspect: ModalityAspect,
    pub level: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Substant {
    pub id: String,
    pub factoid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueNode {
    pub id: String,
    pub content: String,
    pub embedding_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueEdge {
    pub source: String,
    pub target: String,
    pub submodality_id: String,
    pub relationship: String,
    pub weight: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueNetwork {
    pub nodes: Vec<DialogueNode>,
    pub edges: Vec<DialogueEdge>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationResult {
    pub message: String,
    pub insight: Option<String>,
    pub profile_snippet: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Utterance {
    pub id: String,
    pub character_id: String,
    pub substant_id: String,
    pub utterance: String,
    pub insight: Option<String>,
    pub construction_depth: usize,
    pub constructed_from: Option<String>,
    pub used_aspect: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UtterancesWrapper {
    pub utterances: Vec<Utterance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CampaignData {
    pub campaign: Campaign,
    pub characters: Vec<Character>,
    pub modalities: Vec<ModalityLayer>,
    pub substants: Vec<Substant>,
}

impl CampaignData {
    pub fn find_character(&self, character_id: &str) -> Option<&Character> {
        self.characters.iter().find(|c| c.id == character_id)
    }

    pub fn find_modality_aspect(&self, aspect_id: &str) -> Option<&ModalityAspect> {
        for layer in &self.modalities {
            if let Some(aspect) = layer.aspects.iter().find(|a| a.id == aspect_id) {
                return Some(aspect);
            }
        }
        None
    }

    pub fn get_modality_context(&self, aspect_id: &str) -> String {
        if let Some(aspect) = self.find_modality_aspect(aspect_id) {
            format!("Modality Layer: {} - {}", aspect.id, aspect.description)
        } else {
            String::new()
        }
    }
}
