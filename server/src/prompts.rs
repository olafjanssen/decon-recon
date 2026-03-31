pub fn get_dialogue_response_prompt(
    character_profile: &str,
    _communication_style: &str,
    _ideal_response: &str,
    user_message: &str,
) -> String {
    format!(
        "Response prompt for {} with message: {}",
        character_profile, user_message
    )
}
