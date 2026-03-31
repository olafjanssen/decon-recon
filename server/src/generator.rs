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

    pub async fn generate_construction(
        &self,
        campaign_data: &CampaignData,
        character_id: &str,
        aspect_id: &str,
        message: &str,
    ) -> Result<GenerationResult, AppError> {
        let character = campaign_data
            .find_character(character_id)
            .ok_or_else(|| AppError::CharacterNotFound(character_id.to_string()))?;

        let aspect = campaign_data
            .find_modality_aspect(aspect_id)
            .ok_or_else(|| AppError::SubmodalityNotFound(aspect_id.to_string()))?;

        let modality_context = campaign_data.get_modality_context(aspect_id);

        let prompt = prompts::get_construct_translation_prompt(
            &character.name,
            &character.description,
            &aspect.name,
            &aspect.description,
            &modality_context,
            message,
        );

        self.llm_service.generate_structured(&prompt).await
    }
}
