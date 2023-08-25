use replicate_rust::Replicate;

fn initalize_replicate() -> Replicate {
    // let api_key: String = std::env::var("REPLICATE_API_TOKEN")

    let api_key = std::env::var("REPLICATE_API_TOKEN").unwrap_or_else(|_| {
        eprintln!("REPLICATE_API_TOKEN not set");
        std::process::exit(1)
    });

    // Create a new Replicate client.
    Replicate::new(api_key)
}

fn main() {
    let replicate = initalize_replicate();

    // Construct the inputs.
    let mut inputs = std::collections::HashMap::new();
    inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

    let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");

    // Run the model.
    // let result = replicate.run(version, inputs);

    // // Print the result.
    // match result {
    //     Ok(result) => println!("Success : {:?}", result.output),
    //     Err(e) => println!("Error : {}", e),
    // }

    let mut prediction = replicate.predictions.create(version, inputs);
    println!("Prediction : {:?}", prediction.status);
    let _ = prediction.reload();
    println!("Prediction : {:?}", prediction.status);
    let _ = prediction.cancel();
    println!("Predictions : {:?}", prediction.status);
    println!("Predictionss : {:?}", prediction.status);

    match prediction.wait() {
        Ok(result) => println!("Success : {:?}", result.output),
        Err(e) => println!("Error : {}", e),
    }

    // match replicate.predictions.list() {
    //     Ok(result) => println!("Success : {:?}", result),
    //     Err(e) => println!("Error : {}", e),
    // }

    // match replicate
    //     .models
    //     .get(String::from("replicate"), String::from("hello-world"))
    // {
    //     Ok(result) => println!("Success : {:?}", result),
    //     Err(e) => println!("Error : {}", e),
    // };

    // match replicate
    //     .models
    //     .versions
    //     .list(String::from("replicate"), String::from("hello-world"))
    // {
    //     Ok(result) => println!("Success : {:?}", result),
    //     Err(e) => println!("Error : {}", e),
    // };

    // match replicate.models.versions.get(
    //     String::from("kvfrans"),
    //     String::from("clipdraw"),
    //     String::from("5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b"),
    // ) {
    //     Ok(result) => println!("Success : {:?}", result),
    //     Err(e) => println!("Error : {}", e),
    // }

    // match replicate.collection.get(String::from("audio-generation")) {
    //     Ok(result) => println!("Success : {:?}", result),
    //     Err(e) => println!("Error : {}", e),
    // }

    // match replicate.collection.list() {
    //     Ok(result) => println!("Success : {:?}", result),
    //     Err(e) => println!("Error : {}", e),
    // }
}
