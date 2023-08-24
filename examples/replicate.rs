use replicate_rust::Replicate;

fn initalize_replicate() -> Replicate {
    // let api_key: String = std::env::var("REPLICATE_API_TOKEN")

    let api_key = std::env::var("REPLICATE_API_TOKEN").unwrap_or_else(|_| {
        eprintln!("REPLICATE_API_TOKEN not set");
        std::process::exit(1)
    });

    // Create a new Replicate client.
    Replicate::new(
        api_key,
        format!("replicate-rust/{}", env!("CARGO_PKG_VERSION")),
        String::from("https://api.replicate.com/v1/predictions"),
    )
}

fn main() {
    let replicate = initalize_replicate();

    // Construct the inputs.
    let mut inputs = std::collections::HashMap::new();
    inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

    // Run the model.
    let result = replicate.run("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478".to_string(), inputs);

    // Print the result.
    match result {
        Ok(result) => println!("Success : {:?}", result.output),
        Err(e) => println!("Error : {}", e),
    }
}
