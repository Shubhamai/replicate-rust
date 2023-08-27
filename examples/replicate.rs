use replicate_rust::{config::Config, errors::ReplicateError, Replicate};

fn main() -> Result<(), ReplicateError> {
    // let config = Config::default();
    let config = Config {
        auth: String::from("Test Token"),
        ..Default::default()
    };
    let replicate = Replicate::new(config);

    // Creating the inputs
    let mut inputs = std::collections::HashMap::new();
    inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

    let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";

    // Run the model.
    let result = replicate.run(version, inputs)?;

    // Print the result
    println!("Success : {:?}", result.output);

    // Run the model.
    // let result = replicate.run(version, inputs)?;

    // // Print the result.
    // println!("Success : {:?}", result.output);

    // let mut prediction = replicate.predictions.create(version, inputs);
    // println!("Prediction : {:?}", prediction.status);
    // // let _ = prediction.cancel();
    // println!("Predictions : {:?}", prediction.status);
    // println!("Predictionss : {:?}", prediction.status);
    // let _ = prediction.reload();
    // println!("Prediction : {:?}", prediction.logs);

    // println!("Prediction : {:?}", prediction.wait()?);

    // let prediction_list = replicate.predictions.list()?;
    // println!("Prediction Lists : {:?}", prediction_list);

    // let model = replicate.models.get("replicate", "hello-world")?;
    // println!("Model : {:?}", model);

    // let model = replicate
    //     .models
    //     .versions
    //     .get("replicate", "hello-world", "latest")?;
    // println!("Model : {:?}", model);

    // let model = replicate.models.versions.get(
    //     "kvfrans",
    //     "clipdraw",
    //     "5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b",
    // )?;
    // println!("Model : {:?}", model);

    // let collection = replicate.collections.get("audio-generation")?;
    // println!("Collection : {:?}", collection);

    // let collection_list = replicate.collections.list()?;
    // println!("Collection List : {:?}", collection_list);

    Ok(())
}
