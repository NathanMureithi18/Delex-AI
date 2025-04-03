use ic_cdk::update;
use ic_cdk_macros::*;
use std::cell::RefCell;
use candid::{CandidType};
use serde::{Deserialize};

// Define our own ChatMessage struct since we can't use ic_llm directly
#[derive(CandidType, Clone, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

// Thread-local storage for constitution chunks
thread_local! {
    static CONSTITUTION_CHUNKS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

// Initialize with preloaded constitution
#[init]
fn init() {
    CONSTITUTION_CHUNKS.with(|chunks| {
        let mut chunks_mut = chunks.borrow_mut();
        
        // Preamble
        chunks_mut.push("LAWS OF KENYA THE CONSTITUTION OF KENYA Revised Edition 2022 Printed and Published by the Government Printer Nairobi".to_string());
        
        // Chapter One
        chunks_mut.push("CHAPTER ONE - SOVEREIGNTY OF THE PEOPLE AND SUPREMACY OF THIS CONSTITUTION
Article 1. Sovereignty of the people.
Article 2. Supremacy of this Constitution.
Article 3. Defence of this Constitution.".to_string());
        
        // Chapter Two
        chunks_mut.push("CHAPTER TWO - THE REPUBLIC
Article 4. Declaration of the Republic.
Article 5. Territory of Kenya.
Article 6. Devolution and access to services.
Article 7. National, official and other languages.
Article 8. State and religion.
Article 9. National symbols and national days.
Article 10. National values and principles of governance.
Article 11. Culture.".to_string());
        
        // Chapter Three
        chunks_mut.push("CHAPTER THREE - CITIZENSHIP
Article 12. Entitlements of citizens.
Article 13. Retention and acquisition of citizenship.
Article 14. Citizenship by birth.
Article 15. Citizenship by registration.
Article 16. Dual citizenship.
Article 17. Revocation of citizenship.
Article 18. Legislation on citizenship.".to_string());
        
        // Chapter Four - Part 1
        chunks_mut.push("CHAPTER FOUR - THE BILL OF RIGHTS
PART 1 - GENERAL PROVISIONS TO THE BILL OF RIGHTS
Article 19. Rights and fundamental freedoms.
Article 20. Application of Bill of Rights.
Article 21. Implementation of rights and fundamental freedoms.
Article 22. Enforcement of Bill of Rights.
Article 23. Authority of courts to uphold and enforce the Bill of Rights.
Article 24. Limitation of rights and fundamental freedoms.
Article 25. Fundamental Rights and freedoms that may not be limited.".to_string());
        
        // Add more chapters as needed
    });
    
    ic_cdk::println!("Constitution preloaded successfully with {} chunks", 
        CONSTITUTION_CHUNKS.with(|c| c.borrow().len()));
}

// Save constitution to stable memory before upgrades
#[pre_upgrade]
fn pre_upgrade() {
    CONSTITUTION_CHUNKS.with(|chunks| {
        let chunks_data = chunks.borrow();
        match ic_cdk::storage::stable_save((chunks_data.clone(),)) {
            Ok(_) => (),
            Err(e) => ic_cdk::println!("Failed to save to stable memory: {}", e),
        }
    });
}

// Restore constitution from stable memory after upgrades
#[post_upgrade]
fn post_upgrade() {
    match ic_cdk::storage::stable_restore::<(Vec<String>,)>() {
        Ok((chunks_data,)) => {
            CONSTITUTION_CHUNKS.with(|chunks| {
                *chunks.borrow_mut() = chunks_data;
            });
        }
        Err(e) => {
            // Initialize with default data instead of trapping
            ic_cdk::println!("Failed to restore from stable memory: {}", e);
            init(); // Call initialization function
        }
    }
}

// Function to find relevant constitution sections
fn find_relevant_sections(query: &str) -> String {
    let mut result = String::new();
    let keywords = extract_keywords(query);
    
    CONSTITUTION_CHUNKS.with(|chunks| {
        for chunk in chunks.borrow().iter() {
            // Check if any keyword is in the chunk
            if keywords.iter().any(|keyword| 
                chunk.to_lowercase().contains(&keyword.to_lowercase())) {
                result.push_str(chunk);
                result.push_str("\n\n");
            }
        }
    });
    
    if result.is_empty() {
        "No specific constitutional provisions found for this query.".to_string()
    } else {
        result
    }
}

// Helper function to extract keywords from a query
fn extract_keywords(query: &str) -> Vec<String> {
    // Add legal terminology focus
    let legal_terms = ["rights", "freedom", "citizen", "court", "law", "government", "constitution"];
    let common_words = ["the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for"];
    
    let mut keywords = query.split_whitespace()
        .filter(|word| word.len() > 2 && !common_words.contains(&word.to_lowercase().as_str()))
        .map(|word| word.to_string())
        .collect::<Vec<String>>();
    
    // Add legal terms if they appear in the query but weren't caught by the basic extraction
    for term in legal_terms {
        if query.to_lowercase().contains(term) && 
           !keywords.iter().any(|k| k.to_lowercase() == term) {
            keywords.push(term.to_string());
        }
    }
    
    keywords
}

// Function to limit context length
fn get_limited_context(sections: &str, max_chars: usize) -> String {
    if sections.len() <= max_chars {
        return sections.to_string();
    }
    
    // Simple truncation with ellipsis
    let truncated = &sections[0..max_chars];
    format!("{}...\n(Additional relevant sections omitted for brevity)", truncated)
}

// Original prompt function - simplified to just return the constitution info
#[update]
async fn prompt(prompt_str: String) -> String {
    let relevant_sections = find_relevant_sections(&prompt_str);
    
    if relevant_sections.contains("No specific constitutional provisions") {
        format!("I couldn't find specific information about '{}' in the Kenyan Constitution. Please try a different query related to Kenyan constitutional law.", prompt_str)
    } else {
        format!("Here's what the Kenyan Constitution says about '{}':\n\n{}", prompt_str, relevant_sections)
    }
}

// Enhanced prompt with constitution context
#[update]
async fn constitution_prompt(prompt_str: String) -> String {
    let relevant_sections = find_relevant_sections(&prompt_str);
    let limited_context = get_limited_context(&relevant_sections, 2000); // Limit to 2000 chars
    
    if relevant_sections.contains("No specific constitutional provisions") {
        format!("I couldn't find specific information about '{}' in the Kenyan Constitution. Please try a different query related to Kenyan constitutional law.", prompt_str)
    } else {
        format!("Based on the Kenyan Constitution:\n\n{}\n\nRegarding your question about {}: The constitution provides these relevant sections that address your query.", limited_context, prompt_str)
    }
}

// Original chat function
#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    if messages.is_empty() {
        return "Please provide at least one message.".to_string();
    }
    
    // Get the last user message
    let last_message = messages.last()
        .map(|msg| &msg.content)
        .map_or("", |s| s); // Use map_or instead of unwrap_or;
    
    // Use the prompt function to get a response
    prompt(last_message.to_string()).await
}

// Enhanced chat function with constitution context
#[update]
async fn constitution_chat(messages: Vec<ChatMessage>) -> String {
    if messages.is_empty() {
        return "Please provide at least one message.".to_string();
    }
    
    // Get the last user message
    let last_message = messages.last()
        .filter(|msg| msg.role == "user")
        .map(|msg| &msg.content)
        .map_or("", |s| s); // Use map_or instead of unwrap_or;
    
    // Use the constitution_prompt function to get a response
    constitution_prompt(last_message.to_string()).await
}

// Query function to get all constitution chunks (for debugging)
#[query]
fn get_all_constitution_chunks() -> Vec<String> {
    CONSTITUTION_CHUNKS.with(|chunks| chunks.borrow().clone())
}

// Query function to search constitution by keyword
#[query]
fn search_constitution(keyword: String) -> Vec<String> {
    let mut results = Vec::new();
    
    CONSTITUTION_CHUNKS.with(|chunks| {
        for chunk in chunks.borrow().iter() {
            if chunk.to_lowercase().contains(&keyword.to_lowercase()) {
                results.push(chunk.clone());
            }
        }
    });
    
    results
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();