use crate::storage::CONSTITUTION_CHUNKS;

pub fn find_relevant_sections(query: &str) -> String {
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

pub fn extract_keywords(query: &str) -> Vec<String> {
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

pub fn get_limited_context(sections: &str, max_chars: usize) -> String {
    if sections.len() <= max_chars {
        return sections.to_string();
    }
    
    // Simple truncation with ellipsis
    let truncated = &sections[0..max_chars];
    format!("{}...\n(Additional relevant sections omitted for brevity)", truncated)
}

pub fn search_constitution(keyword: String) -> Vec<String> {
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