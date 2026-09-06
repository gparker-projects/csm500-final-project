//! # nle (Natural Language Engine)
//! 
//! Structs and functions within the nle module
//!
//!    CSM500 Project (April - October 2026)
//!      Graham Parker (Student ID: 240120522)
//! 
//! REFERENCES
//! 
//! ## Overview
//! This module provides struts and classes that will work with data and the Natural Language Model (NLM) for the application.
//! This not only handles user-provided prompts, but also the work of initiating and calling the Machine Learning/NLM engine.
//!
 
use tracing;
use ndarray::{Ix2, Axis}; 
use ort::{
	Error,
	session::{Session, builder::GraphOptimizationLevel},
	value::TensorRef
};
use tokenizers::Tokenizer;

// Refs for ML code:
//   https://ort.pyke.io/#load-your-model
//   https://github.com/pykeio/ort/blob/main/examples/sentence-transformers/semantic-similarity.rs
//   S. Lyu and A. Rzeznik, Practical Rust Projects: Build Serverless, AI, Machine Learning, Embedded, Game, and Web Applications. Berkeley, CA: Apress, 2023. doi: DOI:%2010.1007/978-1-4842-9331-7.
//  
pub struct NaturalLanguageEngine {
    session: ort::session::Session,
    tokenizer: tokenizers::Tokenizer,
}

impl NaturalLanguageEngine {

    //
    // cargo_manifest_dir should be: env!("CARGO_MANIFEST_DIR")
    //
    pub async fn new(model_file_path: &str, tokenizer_file_path: &str) -> Self {
        tracing::debug!("NaturalLanguageEngine::new()");
        tracing::debug!("..load model for session: {}", model_file_path);
        tracing::debug!("..load tokenizer: {}", tokenizer_file_path);
        
        NaturalLanguageEngine {
            session: {
                Session::builder().expect("Session could not be established")
                  .with_optimization_level(GraphOptimizationLevel::Level1).expect("No Session")
                  .with_intra_threads(1).expect("Insufficient threads")
                  .commit_from_file(model_file_path ).expect("File could not be accessed")
            },
            tokenizer: {
                Tokenizer::from_file(  tokenizer_file_path  ).unwrap()
            },
        }
    }

    pub async fn get_classifier_rankings(&mut self, inputs: Vec<String> ) -> Vec< (String, f32) > {
      // println!("cwd: {:?}", std::env::current_dir().expect("Current dir could not be accessed"));
      // let canonical = std::fs::canonicalize("all-MiniLM-L6-v2.onnx").expect("File could not be accessed"); // errors if it doesn't exist
      // println!("resolved: {:?}", canonical);
      tracing::debug!("NaturalLanguageEngine::get_classifier_rankings()");
      println!("NaturalLanguageEngine::get_classifier_rankings()");

      let mut results: Vec< (String, f32) > = vec![]; // assemble all results into vector to return

      // Encode our input strings. `encode_batch` will pad each input to be the same length.
      let encodings = self.tokenizer.encode_batch(inputs.clone(), false).map_err(|e| Error::new(e.to_string())).expect("String could not be tokenized");

      // Get the padded length of each encoding.
      let padded_token_length = encodings[0].len();

      // Get our token IDs & mask as a flattened array.
      let ids: Vec<i64> = encodings.iter().flat_map(|e| e.get_ids().iter().map(|i| *i as i64)).collect();
      let mask: Vec<i64> = encodings.iter().flat_map(|e| e.get_attention_mask().iter().map(|i| *i as i64)).collect();

      // Convert our flattened arrays into 2-dimensional tensors of shape [N, L].
      let a_ids = TensorRef::from_array_view(([inputs.len(), padded_token_length], &*ids)).expect("Tensor (ids) could not be flattened");
      let a_mask = TensorRef::from_array_view(([inputs.len(), padded_token_length], &*mask)).expect("Tensor (mask) could not be flattened");

      // Run the model.
      let outputs = self.session.run(ort::inputs![a_ids, a_mask]).expect("Outputs could not be retrieved from session");

      // Extract our embeddings tensor and convert it to a strongly-typed 2-dimensional array.
      let embeddings = outputs[1].try_extract_array::<f32>().expect("Embeddings could not be extracted from array").into_dimensionality::<Ix2>().unwrap();

      //println!("Similarity for '{}'", inputs[0]); // the first item in the list is the item being compared

      let query = embeddings.index_axis(Axis(0), 0);      
      for (embeddings, sentence) in embeddings.axis_iter(Axis(0)).zip(inputs.iter()).skip(1) {
          // Calculate cosine similarity against the 'query' sentence.
          let dot_product: f32 = query.iter().zip(embeddings.iter()).map(|(a, b)| a * b).sum();
          //tracing::info!("\t'{}': {:.1}%", sentence, dot_product * 100.);

          // push each result comparison into our results
          results.push( (sentence.to_string(), dot_product)  );
      }

      // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
      results.sort_unstable_by(|a, b| b.1.total_cmp(&a.1));

      tracing::info!("..Sorted results:");
      //println!("..Sorted results:");
      //for item in results.clone(){
      //    tracing::info!("{} ({:.1}%)", item.0, item.1 * 100.);
      //    println!("{} ({:.1}%)", item.0, item.1 * 100.);
      //}

      results
    }
}