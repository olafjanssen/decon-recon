pub mod campaign;
pub mod error;
pub mod generator;
pub mod llm;
pub mod models;
pub mod prompts;

use campaign::{campaign_exists, list_campaigns, load_campaign};
use clap::{Parser, Subcommand};
use generator::UtteranceGenerator;
use llm::LlmProvider;
use std::path::PathBuf;

fn extract_message_and_insight(
    response: &str,
    fallback_insight: Option<&str>,
) -> (String, Option<String>) {
    // Clean up the response - remove markdown code blocks if present
    let clean_response = response
        .trim()
        .strip_prefix("```json")
        .unwrap_or(response)
        .strip_suffix("```")
        .unwrap_or(response)
        .trim();

    // Try to parse as JSON
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(clean_response) {
        let message = json_value["message"]
            .as_str()
            .unwrap_or(clean_response)
            .to_string();
        let insight = json_value["insight"].as_str().map(|s| s.to_string());
        (message, insight)
    } else {
        // If not valid JSON, use the whole response as message
        (
            clean_response.to_string(),
            fallback_insight.map(|s| s.to_string()),
        )
    }
}

fn get_preferential_modalities(character: &crate::models::Character) -> Vec<&str> {
    character
        .preferential_modalities
        .iter()
        .map(|s| s.as_str())
        .collect()
}

#[derive(Parser)]
#[command(name = "decon-recon-server")]
#[command(about = "A CLI tool for generating campaign utterances for Decon-Recon game")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to campaign data directory
    #[arg(short, long, default_value = "../data/campaign")]
    data_path: PathBuf,

    /// LLM provider to use (mistral or ollama)
    #[arg(long, default_value = "ollama")]
    provider: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Construct a message with a specific aspect for a character
    Construct {
        /// Campaign ID
        campaign_id: String,
        /// Character ID
        character_id: String,
        /// Message to process
        message: String,
        /// Aspect ID to add
        #[arg(short, long)]
        aspect: String,
    },

    /// List available campaigns
    ListCampaigns,

    /// Show campaign details
    ShowCampaign {
        /// Campaign ID
        campaign_id: String,
    },

    /// Create an utterance chain using character's preferential modalities
    Chain {
        /// Campaign ID
        campaign_id: String,
        /// Character ID to start with
        character_id: String,
        /// Substant ID to start from
        substant_id: String,
        /// Number of utterances to generate in chain (default: all preferential modalities)
        #[arg(short, long)]
        length: Option<usize>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Determine LLM provider
    let provider = match cli.provider.as_str() {
        "mistral" => LlmProvider::Mistral,
        "ollama" => LlmProvider::Ollama,
        _ => {
            eprintln!("Invalid provider. Using ollama as default.");
            LlmProvider::Ollama
        }
    };

    let data_path = cli.data_path.to_str().unwrap_or("../data/campaign");

    match &cli.command {
        Commands::ListCampaigns => match list_campaigns(data_path) {
            Ok(campaigns) => {
                println!("Available campaigns:");
                for campaign in campaigns {
                    println!("- {}", campaign);
                }
            }
            Err(e) => eprintln!("Error listing campaigns: {}", e),
        },
        Commands::ShowCampaign { campaign_id } => {
            if !campaign_exists(campaign_id, data_path) {
                eprintln!("Campaign {} not found", campaign_id);
                return Ok(());
            }

            match load_campaign(campaign_id, data_path) {
                Ok(campaign_data) => {
                    println!("Campaign: {}", campaign_data.campaign.title);
                    println!("ID: {}", campaign_data.campaign.id);
                    println!("Hub: {}", campaign_data.campaign.hub_location);
                    println!(
                        "
Characters:"
                    );
                    for char in &campaign_data.characters {
                        println!("- {} ({}): {}", char.name, char.id, char.location);
                    }
                }
                Err(e) => eprintln!("Error loading campaign: {}", e),
            }
        }
        Commands::Chain {
            campaign_id,
            character_id,
            substant_id,
            length,
        } => {
            if !campaign_exists(campaign_id, data_path) {
                eprintln!("Campaign {} not found", campaign_id);
                return Ok(());
            }

            let campaign_data = match load_campaign(campaign_id, data_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error loading campaign: {}", e);
                    return Ok(());
                }
            };

            // Find the character
            let character = campaign_data
                .characters
                .iter()
                .find(|c| c.id == *character_id);

            if character.is_none() {
                eprintln!("Character {} not found in campaign", character_id);
                return Ok(());
            }

            let character = character.unwrap();

            // Check if substant exists
            let substant = campaign_data
                .substants
                .iter()
                .find(|s| s.id == *substant_id);

            if substant.is_none() {
                eprintln!("Substant {} not found in campaign", substant_id);
                return Ok(());
            }

            let generator = match UtteranceGenerator::new(provider) {
                Ok(gen) => gen,
                Err(e) => {
                    eprintln!("Error creating generator: {}", e);
                    return Ok(());
                }
            };

            // Get preferential modalities with levels and sort by level
            let modalities_with_levels = campaign::get_modalities_with_levels(
                &campaign_data,
                &get_preferential_modalities(character),
            );

            // Default length: use all modalities if not specified
            let chain_length = length.unwrap_or_else(|| modalities_with_levels.len());

            // Start with the substant factoid
            let mut current_message = substant.unwrap().factoid.clone();
            let mut previous_utterance_id: Option<String> = None;

            for i in 0..chain_length {
                // Use modality in order (already sorted by level)
                let aspect = &modalities_with_levels[i % modalities_with_levels.len()]
                    .aspect
                    .id;

                match generator
                    .generate_construction(&campaign_data, character_id, aspect, &current_message)
                    .await
                {
                    Ok(result) => {
                        // Extract message and insight from JSON response
                        let (message_text, insight_text) =
                            extract_message_and_insight(&result.message, result.insight.as_deref());

                        // Create and save utterance
                        let utterance = campaign::create_utterance_with_insight(
                            character_id,
                            substant_id,
                            &message_text,
                            insight_text.as_deref(),
                            previous_utterance_id.as_deref(),
                            Some(aspect),
                        );

                        if let Err(e) = campaign::save_utterance(campaign_id, data_path, &utterance)
                        {
                            eprintln!("Error saving utterance {}: {}", i + 1, e);
                            return Ok(());
                        }

                        println!("Utterance {}:", i + 1);
                        println!("ID: {}", utterance.id);
                        println!("Character: {}", character_id);
                        println!(
                            "Level {} Aspect: {}",
                            modalities_with_levels[i % modalities_with_levels.len()].level,
                            aspect
                        );
                        println!("Message: {}", message_text);
                        if let Some(insight) = &utterance.insight {
                            println!("Insight: {}", insight);
                        }

                        // Update for next iteration
                        current_message = message_text;
                        previous_utterance_id = Some(utterance.id.clone());
                    }
                    Err(e) => {
                        eprintln!("Error generating utterance {}: {}", i + 1, e);
                        return Ok(());
                    }
                }
            }

            println!("\nChain of {} utterances created successfully using character's preferential modalities!", chain_length);
        }
        Commands::Construct {
            campaign_id,
            character_id,
            message,
            aspect,
        } => {
            if !campaign_exists(campaign_id, data_path) {
                eprintln!("Campaign {} not found", campaign_id);
                return Ok(());
            }

            let campaign_data = match load_campaign(campaign_id, data_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error loading campaign: {}", e);
                    return Ok(());
                }
            };

            let generator = match UtteranceGenerator::new(provider) {
                Ok(gen) => gen,
                Err(e) => {
                    eprintln!("Error creating generator: {}", e);
                    return Ok(());
                }
            };

            match generator
                .generate_construction(&campaign_data, character_id, &aspect, message)
                .await
            {
                Ok(result) => {
                    println!("Generated construction:");
                    println!("Message: {}", result.message);
                    if let Some(insight) = result.insight {
                        println!("Insight: {}", insight);
                    }
                    if let Some(profile_snippet) = result.profile_snippet {
                        println!("Profile snippet: {}", profile_snippet);
                    }
                }
                Err(e) => eprintln!("Error generating construction: {}", e),
            }
        }
    }

    Ok(())
}
