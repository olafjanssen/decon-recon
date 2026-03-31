use crate::error::AppError;
use crate::llm::{LlmProvider, LlmService};
use crate::models::{CampaignData, GenerationResult};
use crate::prompts;

pub struct UtteranceGenerator {
    pub llm_service: LlmService,
}

impl UtteranceGenerator {
    pub fn new(provider: LlmProvider) -> Result<Self, AppError> {
        let llm_service = LlmService::new(provider)?;
        Ok(Self { llm_service })
    }

    pub async fn generate_response(
        &self,
        campaign_data: &CampaignData,
        character_id: &str,
        message: &str,
    ) -> Result<String, AppError> {
        let character = campaign_data
            .find_character(character_id)
            .ok_or_else(|| AppError::CharacterNotFound(character_id.to_string()))?;

        let prompt = prompts::get_dialogue_response_prompt(&character.description, "", "", message);

        self.llm_service.generate_text(&prompt).await
    }

    pub async fn generate_construction(
        &self,
        campaign_data: &CampaignData,
        character_id: &str,
        submodality_id: &str,
        message: &str,
    ) -> Result<GenerationResult, AppError> {
        let character = campaign_data
            .find_character(character_id)
            .ok_or_else(|| AppError::CharacterNotFound(character_id.to_string()))?;

        let modality_aspect = campaign_data
            .find_modality_aspect(submodality_id)
            .ok_or_else(|| AppError::SubmodalityNotFound(submodality_id.to_string()))?;

        let _modality_context = campaign_data.get_modality_context(submodality_id);

        let prompt = format!(
            "Construct message for {} using submodality {}: {}",
            character.name, modality_aspect.name, message
        );

        self.llm_service.generate_structured(&prompt).await
    }

    pub async fn generate_deconstruction(
        &self,
        campaign_data: &CampaignData,
        character_id: &str,
        submodality_id: &str,
        message: &str,
    ) -> Result<GenerationResult, AppError> {
        let character = campaign_data
            .find_character(character_id)
            .ok_or_else(|| AppError::CharacterNotFound(character_id.to_string()))?;

        let modality_aspect = campaign_data
            .find_modality_aspect(submodality_id)
            .ok_or_else(|| AppError::SubmodalityNotFound(submodality_id.to_string()))?;

        let _modality_context = campaign_data.get_modality_context(submodality_id);

        let prompt = format!(
            "Deconstruct message for {} removing submodality {}: {}",
            character.name, modality_aspect.name, message
        );

        self.llm_service.generate_structured(&prompt).await
    }

    pub async fn generate_response_mock(
        &self,
        campaign_data: &CampaignData,
        character_id: &str,
        message: &str,
    ) -> Result<String, AppError> {
        let character = campaign_data
            .find_character(character_id)
            .ok_or_else(|| AppError::CharacterNotFound(character_id.to_string()))?;

        let prompt = prompts::get_dialogue_response_prompt(&character.description, "", "", message);

        self.llm_service.generate_text_mock(&prompt).await
    }
}
