pub fn get_construct_translation_prompt(
    character_name: &str,
    character_profile: &str,
    aspect_name: &str,
    aspect_description: &str,
    layer_context: &str,
    message: &str,
) -> String {
    format!(
        "You are a language construction expert. Your task is to analyse a given text and rewrite it such a way that it retains the core meaning but adds the aspect in the text related to the given aspect.

Optional profile information about the recipient ({}): {}
Aspect name: {}
Aspect description: {}

{}

Guidelines:
- Add ONLY elements specific to the requested aspect's modality layer.
- DO NOT modify aspects from other modality layers - preserve all elements from modalities other than the target modality.
- Only add elements within the target modality layer (the layer that this aspect belongs to).
- BE CONCISE and stay below 100 words
- DO NOT USE markdown formatting.
- Preserve all other semantic elements from all other modality layers.
- Keep the message coherent and natural.
- Maintain readability and flow.
- Ensure the core meaning remains intact.
- Be conservative - only add what is clearly part of the target modality layer, not elements from other layers.
- Rewrite the message as much as possible, don't simply add a phrase at the end.

Also provide snippet from the given profile or required addition to the profile to rewrite the message as short phrase such as:
\"likes Elephants\", or \"has a Southern accent\".

IMPORTANT: You must respond with ONLY a valid JSON object in the following format. Do not include any other text, explanations, or markdown formatting:

{{
  \"message\": \"the rewritten message\",
  \"insight\": \"insight about character profile as a short phrase, based on the aspect and the message\"
}}

Please construct this content:
{}",
        character_name, character_profile, aspect_name, aspect_description, layer_context, message
    )
}
