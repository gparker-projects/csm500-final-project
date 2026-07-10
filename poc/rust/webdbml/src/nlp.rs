use std::path::Path;

use ndarray::{Ix2}; //Axis
use ort::{
	Error,
	session::{Session, builder::GraphOptimizationLevel},
	value::TensorRef
};
use tokenizers::Tokenizer;
//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


//use serde::{Deserialize, Serialize};
//https://github.com/pykeio/ort/blob/main/examples/sentence-transformers/semantic-similarity.rs

pub struct NLP {
    
}

impl NLP {

    //pub async fn new(db_url: &str) -> Self {

   // }

    pub async fn execute( &self ) -> Result<(), std::io::Error> {

	  // Load our model
	  let mut session = Session::builder().expect("Session could not be established")
		.with_optimization_level(GraphOptimizationLevel::Level1).expect("No Session")
		.with_intra_threads(1).expect("Insufficient threads")
		.commit_from_file("../../../../../data/all-MiniLM-L6-v2.onnx").expect("File could not be accessed");

      // Load the tokenizer and encode the text.
      let tokenizer = Tokenizer::from_file(Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join("tokenizer.json")).unwrap();
      let inputs = vec!["The weather outside is lovely.", "It's so sunny outside!", "She drove to the stadium."];

      // Encode our input strings. `encode_batch` will pad each input to be the same length.
      let encodings = tokenizer.encode_batch(inputs.clone(), false).map_err(|e| Error::new(e.to_string())).expect("String could not be tokenized");

      // Get the padded length of each encoding.
      let padded_token_length = encodings[0].len();

      // Get our token IDs & mask as a flattened array.
      let ids: Vec<i64> = encodings.iter().flat_map(|e| e.get_ids().iter().map(|i| *i as i64)).collect();
      let mask: Vec<i64> = encodings.iter().flat_map(|e| e.get_attention_mask().iter().map(|i| *i as i64)).collect();

      // Convert our flattened arrays into 2-dimensional tensors of shape [N, L].
      let a_ids = TensorRef::from_array_view(([inputs.len(), padded_token_length], &*ids)).expect("Tensor (ids) could not be flattened");
      let a_mask = TensorRef::from_array_view(([inputs.len(), padded_token_length], &*mask)).expect("Tensor (mask) could not be flattened");

      // Run the model.
      let outputs = session.run(ort::inputs![a_ids, a_mask]).expect("Outputs could not be retrieved from session");

      // Extract our embeddings tensor and convert it to a strongly-typed 2-dimensional array.
      let embeddings = outputs[1].try_extract_array::<f32>().expect("Embeddings could not be extracted from array").into_dimensionality::<Ix2>().unwrap();

      format!("Similarity for '{}'", inputs[0]);

      Ok(())
    }

}