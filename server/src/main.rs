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
    /// Generate an utterance for a character
    Generate {
        /// Campaign ID
        campaign_id: String,
        /// Character ID
        character_id: String,
        /// Message to process
        message: String,
        /// Action: respond, construct, or deconstruct
        #[arg(default_value = "respond")]
        action: String,
        /// Submodality ID (for construct/deconstruct)
        #[arg(short, long)]
        submodality: Option<String>,
    },

    /// List available campaigns
    ListCampaigns,

    /// Show campaign details
    ShowCampaign {
        /// Campaign ID
        campaign_id: String,
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
        Commands::Generate {
            campaign_id,
            character_id,
            message,
            action,
            submodality,
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

            match action.as_str() {
                "respond" => {
                    match generator
                        .generate_response(&campaign_data, character_id, message)
                        .await
                    {
                        Ok(response) => {
                            println!("Generated response:");
                            println!("{}", response);
                        }
                        Err(e) => eprintln!("Error generating response: {}", e),
                    }
                }
                "construct" => {
                    if let Some(submodality_id) = submodality {
                        match generator
                            .generate_construction(
                                &campaign_data,
                                character_id,
                                submodality_id,
                                message,
                            )
                            .await
                        {
                            Ok(result) => {
                                println!("Generated construction:");
                                println!("Message: {}", result.message);
                                if let Some(insight) = result.insight {
                                    println!("Insight: {}", insight);
                                }
                            }
                            Err(e) => eprintln!("Error generating construction: {}", e),
                        }
                    } else {
                        eprintln!("Submodality ID required for construct action");
                    }
                }
                "deconstruct" => {
                    if let Some(submodality_id) = submodality {
                        match generator
                            .generate_deconstruction(
                                &campaign_data,
                                character_id,
                                submodality_id,
                                message,
                            )
                            .await
                        {
                            Ok(result) => {
                                println!("Generated deconstruction:");
                                println!("Message: {}", result.message);
                                if let Some(insight) = result.insight {
                                    println!("Insight: {}", insight);
                                }
                            }
                            Err(e) => eprintln!("Error generating deconstruction: {}", e),
                        }
                    } else {
                        eprintln!("Submodality ID required for deconstruct action");
                    }
                }
                _ => eprintln!("Invalid action: {}", action),
            }
        }
    }

    Ok(())
}
