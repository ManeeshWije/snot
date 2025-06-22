use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

use crate::parser::{cosine_similarity, read_file};

mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_file("README.md");

    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()?;

    let sentences = parser::split_sentences(&content);

    let output = model.encode(&sentences)?;

    // use consine similarity to output based on query
    let query = "what are the features of this program?";

    let query_embedding = model.encode(&[query])?;

    // Calculate cosine similarity between the query embedding and each sentence embedding
    // then for each sentence, output the sentence and the next 5 sentences next to it for context
    let mut results: Vec<(String, f32)> = Vec::new();
    for (i, sentence) in sentences.iter().enumerate() {
        let similarity = cosine_similarity(&query_embedding[0], &output[i]);
        results.push((sentence.clone(), similarity));
    }
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("Top 5 results for query '{}':", query);
    for (i, (sentence, similarity)) in results.iter().take(5).enumerate() {
        println!("{}. {} (similarity: {:.4})", i + 1, sentence, similarity);
        let start = sentences.iter().position(|s| s == sentence).unwrap();
        let end = (start + 5).min(sentences.len());
        for context_sentence in &sentences[start + 1..end] {
            println!("   {}", context_sentence);
        }
    }
    Ok(())
}
