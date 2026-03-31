# Decon-Recon Rust CLI Server

A lightweight Rust CLI tool for generating campaign utterances for the Decon-Recon game.

## Features

- Campaign management and inspection
- Character response generation using LLMs
- Modality-based message construction/deconstruction
- Support for Mistral API and Ollama local models
- JSON output for game integration

## Installation

```bash
cd server
cargo build --release
```

## Usage

```bash
# List campaigns
./target/release/decon-recon-server list-campaigns

# Show campaign details
./target/release/decon-recon-server show-campaign town-hall-murder

# Generate response
./target/release/decon-recon-server generate town-hall-murder elias "Hello there"

# Construct with modality
./target/release/decon-recon-server generate town-hall-murder elias "message" construct --submodality emotional:paranoia

# Deconstruct with modality
./target/release/decon-recon-server generate town-hall-murder rosa "message" deconstruct --submodality emotional:defensiveness
```

## Configuration

- `--data-path`: Path to campaign data (default: ../data/campaign)
- `--provider`: LLM provider (mistral or ollama, default: ollama)
- `MISTRAL_API_KEY`: Required for Mistral provider
- `OLLAMA_ENDPOINT`: Ollama server URL

## Architecture

- **models.rs**: Data structures
- **campaign.rs**: Campaign loading
- **llm.rs**: LLM service implementations
- **prompts.rs**: Prompt templates
- **generator.rs**: Core generation logic
- **error.rs**: Error handling

## Data Structure

Expects TOML files in data/campaign/{campaign_id}/ with:
- campaign.toml, characters.toml, modalities.toml, substants.toml

## Examples

```bash
# Basic usage
./target/release/decon-recon-server generate town-hall-murder elias "What happened?"

# With Mistral
MISTRAL_API_KEY=your_key ./target/release/decon-recon-server generate campaign char "message" --provider mistral
```
