use ic_cdk::update;
use ic_cdk_macros::*;

mod types;
mod storage;
mod retrieval;
mod chat;

use types::ChatMessage;

#[init]
fn init() {
    storage::init_constitution();
}

#[pre_upgrade]
fn pre_upgrade() {
    storage::pre_upgrade_handler();
}

#[post_upgrade]
fn post_upgrade() {
    storage::post_upgrade_handler();
}

#[update]
async fn prompt(prompt_str: String) -> String {
    chat::prompt(prompt_str).await
}

#[update]
async fn constitution_prompt(prompt_str: String) -> String {
    chat::constitution_prompt(prompt_str).await
}

#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    chat::chat(messages).await
}

#[update]
async fn constitution_chat(messages: Vec<ChatMessage>) -> String {
    chat::constitution_chat(messages).await
}

#[query]
fn get_all_constitution_chunks() -> Vec<String> {
    storage::get_all_chunks()
}

#[query]
fn search_constitution(keyword: String) -> Vec<String> {
    retrieval::search_constitution(keyword)
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();