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
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// For parsing utterances.toml
use toml;

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
        return (message, insight);
    } else {
        // If not valid JSON, use the whole response as message
        return (
            clean_response.to_string(),
            fallback_insight.map(|s| s.to_string()),
        );
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

    /// Model to use for LLM provider (only applies to ollama)
    #[arg(long, default_value = "mistral-small3.2:latest")]
    model: String,
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

    /// Generate all possible chains of given length with gaps
    AllChains {
        /// Campaign ID
        campaign_id: String,
        /// Character ID to start with
        character_id: String,
        /// Substant ID to start from
        substant_id: String,
        /// Chain length to generate (must start with core layer)
        #[arg(short, long, default_value_t = 5)]
        length: usize,
    },

    /// Generate complete campaign: all chains for all characters and substants
    GenerateCampaign {
        /// Campaign ID
        campaign_id: String,
        /// Chain length for each substant (default: 5)
        #[arg(short, long, default_value_t = 5)]
        length: usize,
    },

    /// Export campaign data to JSON format for Godot
    ExportForGodot {
        /// Campaign ID
        campaign_id: String,
        /// Output JSON file path
        #[arg(short, long, default_value = "campaign_data.json")]
        output: String,
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

            let generator = match UtteranceGenerator::new(provider, &cli.model) {
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

                        // Check if this utterance already exists
                        if let Some(existing_utterance) = campaign::find_existing_utterance(
                            campaign_id,
                            data_path,
                            character_id,
                            substant_id,
                            &message_text,
                            i,
                        ) {
                            println!("Reusing existing utterance: {}", existing_utterance.id);
                            previous_utterance_id = Some(existing_utterance.id.clone());
                            current_message = existing_utterance.utterance.clone();
                            continue; // Skip to next iteration
                        }

                        // Create and save utterance
                        let utterance = campaign::create_utterance_with_insight(
                            character_id,
                            substant_id,
                            &message_text,
                            insight_text.as_deref(),
                            i, // construction_depth
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

            let generator = match UtteranceGenerator::new(provider, &cli.model) {
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
        Commands::AllChains {
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

            let generator = match UtteranceGenerator::new(provider, &cli.model) {
                Ok(gen) => gen,
                Err(e) => {
                    eprintln!("Error creating generator: {}", e);
                    return Ok(());
                }
            };

            // Get modalities with levels and generate all combinations
            let modalities_with_levels = campaign::get_modalities_with_levels(
                &campaign_data,
                &get_preferential_modalities(character),
            );

            if modalities_with_levels.len() < *length {
                eprintln!(
                    "Character only has {} modalities, cannot create chains of length {}",
                    modalities_with_levels.len(),
                    length
                );
                return Ok(());
            }

            let combinations =
                campaign::generate_modality_combinations(&modalities_with_levels, *length);

            println!(
                "Generating {} possible chains of length {}...",
                combinations.len(),
                length
            );

            for (chain_index, modality_chain) in combinations.iter().enumerate() {
                println!("\n=== Chain {} ===", chain_index + 1);

                let mut current_message = substant.unwrap().factoid.clone();
                let mut previous_utterance_id: Option<String> = None;

                for (step, modality_with_level) in modality_chain.iter().enumerate() {
                    let aspect = &modality_with_level.aspect.id;

                    match generator
                        .generate_construction(
                            &campaign_data,
                            character_id,
                            aspect,
                            &current_message,
                        )
                        .await
                    {
                        Ok(result) => {
                            // Extract message and insight from JSON response
                            let (message_text, insight_text) = extract_message_and_insight(
                                &result.message,
                                result.insight.as_deref(),
                            );

                            // Check if this utterance already exists
                            if let Some(existing_utterance) = campaign::find_existing_utterance(
                                campaign_id,
                                data_path,
                                character_id,
                                substant_id,
                                &message_text,
                                step,
                            ) {
                                println!("Reusing existing utterance: {}", existing_utterance.id);
                                previous_utterance_id = Some(existing_utterance.id.clone());
                                current_message = existing_utterance.utterance.clone();
                                continue; // Skip to next iteration
                            }

                            // Create and save utterance
                            let utterance = campaign::create_utterance_with_insight(
                                character_id,
                                substant_id,
                                &message_text,
                                insight_text.as_deref(),
                                step, // construction_depth
                                previous_utterance_id.as_deref(),
                                Some(aspect),
                            );

                            if let Err(e) =
                                campaign::save_utterance(campaign_id, data_path, &utterance)
                            {
                                eprintln!("Error saving utterance: {}", e);
                            }

                            println!(
                                "Step {} (Level {} {}): {}",
                                step + 1,
                                modality_with_level.level,
                                aspect,
                                message_text
                            );

                            // Update for next iteration
                            current_message = message_text;
                            previous_utterance_id = Some(utterance.id.clone());
                        }
                        Err(e) => {
                            eprintln!("Error generating step {}: {}", step + 1, e);
                            break;
                        }
                    }
                }
            }

            println!(
                "\nGenerated {} complete chains with all combinations!",
                combinations.len()
            );
        }

        Commands::GenerateCampaign {
            campaign_id,
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

            let generator = match UtteranceGenerator::new(provider, &cli.model) {
                Ok(gen) => gen,
                Err(e) => {
                    eprintln!("Error creating generator: {}", e);
                    return Ok(());
                }
            };

            println!("Generating complete campaign dialogue network...");
            println!("Characters: {}", campaign_data.characters.len());
            println!("Substants: {}", campaign_data.substants.len());

            let mut total_utterances = 0;
            let mut reused_utterances = 0;

            for character in &campaign_data.characters {
                println!(
                    "
=== Processing character: {} ({}) ===",
                    character.name, character.id
                );

                // Get modalities with levels for this character
                let modalities_with_levels = campaign::get_modalities_with_levels(
                    &campaign_data,
                    &get_preferential_modalities(character),
                );

                if modalities_with_levels.len() < *length {
                    eprintln!(
                        "Character {} only has {} modalities, cannot create chains of length {}",
                        character.name,
                        modalities_with_levels.len(),
                        length
                    );
                    continue;
                }

                let combinations =
                    campaign::generate_modality_combinations(&modalities_with_levels, *length);

                for substant in &campaign_data.substants {
                    println!(
                        "Processing substant: {} ({})...",
                        substant.factoid, substant.id
                    );
                    println!(
                        "Generating {} possible chains of length {}...",
                        combinations.len(),
                        length
                    );

                    for (chain_index, modality_chain) in combinations.iter().enumerate() {
                        println!("\n=== Chain {} ===", chain_index + 1);

                        let mut current_message = substant.factoid.clone();
                        let mut previous_utterance_id: Option<String> = None;

                        for (step, modality_with_level) in modality_chain.iter().enumerate() {
                            let aspect = &modality_with_level.aspect.id;

                            match generator
                                .generate_construction(
                                    &campaign_data,
                                    &character.id,
                                    aspect,
                                    &current_message,
                                )
                                .await
                            {
                                Ok(result) => {
                                    // Extract message and insight from JSON response
                                    let (message_text, insight_text) = extract_message_and_insight(
                                        &result.message,
                                        result.insight.as_deref(),
                                    );

                                    // Check if this utterance already exists
                                    if let Some(existing_utterance) =
                                        campaign::find_existing_utterance(
                                            campaign_id,
                                            data_path,
                                            &character.id,
                                            &substant.id,
                                            &message_text,
                                            step,
                                        )
                                    {
                                        println!(
                                            "  Step {}: Reusing existing utterance {}",
                                            step + 1,
                                            existing_utterance.id
                                        );
                                        reused_utterances += 1;
                                        previous_utterance_id = Some(existing_utterance.id.clone());
                                        current_message = existing_utterance.utterance.clone();
                                        continue;
                                    }

                                    // Create and save new utterance
                                    let utterance = campaign::create_utterance_with_insight(
                                        &character.id,
                                        &substant.id,
                                        &message_text,
                                        insight_text.as_deref(),
                                        step,
                                        previous_utterance_id.as_deref(),
                                        Some(aspect),
                                    );

                                    if let Err(e) =
                                        campaign::save_utterance(campaign_id, data_path, &utterance)
                                    {
                                        eprintln!("Error saving utterance: {}", e);
                                    } else {
                                        println!(
                                            "  Step {} (Level {} {}): {}",
                                            step + 1,
                                            modality_with_level.level,
                                            aspect,
                                            message_text
                                        );
                                        total_utterances += 1;
                                        previous_utterance_id = Some(utterance.id.clone());
                                        current_message = message_text;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error generating step {}: {}", step + 1, e);
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            println!(
                "
=== Campaign Generation Complete ==="
            );
            println!("Total new utterances created: {}", total_utterances);
            println!("Total utterances reused: {}", reused_utterances);
            println!(
                "Total utterances in network: {}",
                total_utterances + reused_utterances
            );
        }

        Commands::ExportForGodot {
            campaign_id,
            output,
        } => {
            if !campaign_exists(&campaign_id, data_path) {
                eprintln!("Campaign {} not found", campaign_id);
                return Ok(());
            }

            let campaign_data = match load_campaign(&campaign_id, data_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error loading campaign: {}", e);
                    return Ok(());
                }
            };

            // Load utterances from the utterances.toml file
            let utterances_path = format!("{}/{}/utterances.toml", data_path, campaign_id);
            let utterances_data = match std::fs::read_to_string(&utterances_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!(
                        "Warning: Could not read utterances file {}: {}",
                        utterances_path, e
                    );
                    String::new()
                }
            };

            // Parse utterances from TOML
            let utterances: Vec<Value> = if utterances_data.is_empty() {
                Vec::new()
            } else {
                match utterances_data.parse::<toml::Value>() {
                    Ok(toml_value) => {
                        if let Some(utterances_array) =
                            toml_value.get("utterances").and_then(|v| v.as_array())
                        {
                            utterances_array.iter().map(|utt| {
                                json!({
                                    "id": utt.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                                    "character_id": utt.get("character_id").and_then(|v| v.as_str()).unwrap_or(""),
                                    "substant_id": utt.get("substant_id").and_then(|v| v.as_str()).unwrap_or(""),
                                    "utterance": utt.get("utterance").and_then(|v| v.as_str()).unwrap_or(""),
                                    "insight": utt.get("insight").and_then(|v| v.as_str()),
                                    "construction_depth": utt.get("construction_depth").and_then(|v| v.as_integer()).map(|n| n as u32),
                                    "used_aspect": utt.get("used_aspect").and_then(|v| v.as_str()),
                                    "constructed_from": utt.get("constructed_from").and_then(|v| v.as_str()),
                                })
                            }).collect()
                        } else {
                            Vec::new()
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not parse utterances.toml: {}", e);
                        Vec::new()
                    }
                }
            };

            // Convert campaign data to Godot-friendly JSON format
            let json_data = json!({
                "campaign": {
                    "id": campaign_data.campaign.id,
                    "title": campaign_data.campaign.title,
                    "hub_location": campaign_data.campaign.hub_location,
                    "description": campaign_data.campaign.description,
                    "introduction": campaign_data.campaign.introduction,
                    "resolution": campaign_data.campaign.resolution,
                },
                "characters": campaign_data.characters.iter().map(|char| json!({
                    "id": char.id,
                    "name": char.name,
                    "location": char.location,
                    "description": char.description,
                    "preferential_modalities": char.preferential_modalities,
                    "secret": char.secret,
                })).collect::<Vec<Value>>(),
                "substants": campaign_data.substants.iter().map(|sub| json!({
                    "id": sub.id,
                    "factoid": sub.factoid,
                })).collect::<Vec<Value>>(),
                "utterances": utterances,
                "modalities": campaign_data.modalities.iter().map(|modality| json!({
                    "level": modality.level,
                    "id": modality.id,
                    "name": modality.name,
                    "layman_name": modality.layman_name,
                    "full_description": modality.full_description,
                    "aspects": modality.aspects.iter().map(|aspect| json!({
                        "id": aspect.id,
                        "name": aspect.name,
                        "layman_name": aspect.layman_name,
                        "description": aspect.description,
                        "icon": aspect.icon,
                    })).collect::<Vec<Value>>(),
                })).collect::<Vec<Value>>(),
            });

            // Write JSON to file
            let mut file = match File::create(&output) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error creating output file {}: {}", output, e);
                    return Ok(());
                }
            };

            if let Err(e) = file.write_all(json_data.to_string().as_bytes()) {
                eprintln!("Error writing to file {}: {}", output, e);
                return Ok(());
            }

            println!(
                "Successfully exported campaign '{}' to {}",
                campaign_id, output
            );
        }
    }
    Ok(())
}
