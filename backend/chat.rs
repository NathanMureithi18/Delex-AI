use crate::types::ChatMessage;
use crate::retrieval::{find_relevant_sections, get_limited_context};

pub async fn prompt(prompt_str: String) -> String {
    let relevant_sections = find_relevant_sections(&prompt_str);
    
    if relevant_sections.contains("No specific constitutional provisions") {
        format!("I couldn't find specific information about '{}' in the Kenyan Constitution. Please try a different query related to Kenyan constitutional law.", prompt_str)
    } else {
        format!("Here's what the Kenyan Constitution says about '{}':\n\n{}", prompt_str, relevant_sections)
    }
}

pub async fn constitution_prompt(prompt_str: String) -> String {
    let relevant_sections = find_relevant_sections(&prompt_str);
    let limited_context = get_limited_context(&relevant_sections, 2000); // Limit to 2000 chars
    
    if relevant_sections.contains("No specific constitutional provisions") {
        format!("I couldn't find specific information about '{}' in the Kenyan Constitution. Please try a different query related to Kenyan constitutional law.", prompt_str)
    } else {
        format!("Based on the Kenyan Constitution:\n\n{}\n\nRegarding your question about {}: The constitution provides these relevant sections that address your query.", limited_context, prompt_str)
    }
}

pub async fn chat(messages: Vec<ChatMessage>) -> String {
    if messages.is_empty() {
        return "Please provide at least one message.".to_string();
    }
    
    // Get the last user message
    let last_message = messages.last()
        .map(|msg| &msg.content)
        .map_or("", |s| s);
    
    // Use the prompt function to get a response
    prompt(last_message.to_string()).await
}

pub async fn constitution_chat(messages: Vec<ChatMessage>) -> String {
    if messages.is_empty() {
        return "Please provide at least one message.".to_string();
    }
    
    // Get the last user message
    let last_message = messages.last()
        .filter(|msg| msg.role == "user")
        .map(|msg| &msg.content)
        .map_or("", |s| s);
    
    // Use the constitution_prompt function to get a response
    constitution_prompt(last_message.to_string()).await
}