use std::cell::RefCell;
use ic_cdk_macros::*;
use ic_cdk::api::call::call;
use candid::Principal;
use ic_llm::{ChatMessage, Model};

// DecideAI canister ID for image processing
const DECIDEAI_CANISTER_ID: &str = "x4kx5-ziaaa-aaaaq-aabeq-cai"; // Make sure this ID is complete

thread_local! {
    static DECIDEAI: RefCell<Option<Principal>> = RefCell::new(None);
}

// Text processing using ic_llm
#[update]
async fn prompt(prompt_str: String) -> String {
    ic_llm::prompt(Model::Llama3_1_8B, prompt_str).await
}

#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    ic_llm::chat(Model::Llama3_1_8B, messages).await
}

// Image processing using DecideAI
#[update]
pub async fn process_image(image: Vec<u8>) -> String {
    let response: Result<(String,), _> = call(
        Principal::from_text(DECIDEAI_CANISTER_ID).expect("Invalid principal ID"),
        "analyzeImage",
        (image,),
    ).await;
    
    match response {
        Ok((result,)) => result,
        Err(e) => format!("Error processing image: {:?}", e)
    }
}

// Combined function that routes to appropriate handler
#[update]
pub async fn process_input(image: Option<Vec<u8>>, text: Option<String>) -> String {
    match (text, image) {
        // Text only - use ic_llm
        (Some(text_prompt), None) if !text_prompt.is_empty() => {
            prompt(text_prompt).await
        },
        
        // Image only - use DecideAI
        (None, Some(img_data)) => {
            process_image(img_data).await
        },
        
        // Both text and image - process separately and combine results
        (Some(text_prompt), Some(img_data)) if !text_prompt.is_empty() => {
            let text_response = prompt(text_prompt).await;
            let image_analysis = process_image(img_data).await;
            format!("Text response: {}\n\nImage analysis: {}", text_response, image_analysis)
        },
        
        // No valid input
        _ => "Please provide either text or an image to process.".to_string()
    }
}

// Export the Candid interface
ic_cdk::export_candid!();