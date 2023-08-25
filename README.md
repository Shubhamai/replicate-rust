[![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]


# Replicate Rust client

This is an Unofficial Rust client for [Replicate](https://replicate.com).

## Install

```sh
cargo add replicate_rust
```

## Authenticate

Grab your token from [replicate.com/account](https://replicate.com/account) and set it as an environment variable:

```
export REPLICATE_API_TOKEN=<your token>
```

## Run a model

```rust
let replicate = Replicate::new();

// Construct the inputs.
let mut inputs = std::collections::HashMap::new();
inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");

// Run the model.
let result = replicate.run(version, inputs);

// Print the result.
match result {
    Ok(result) => println!("Success : {:?}", result.output),
    Err(e) => println!("Error : {}", e),
}
// Some(Array [String("https://pbxt.replicate.delivery/QLDGe2rXuIQ9ByMViQEXrYCkKfDi9I3YWAzPwWsDZWMXeN7iA/out-0.png")])
```


## Run a model in the background

You can start a model and run it in the background:

```rust
let replicate = Replicate::new();

// Construct the inputs.
let mut inputs = std::collections::HashMap::new();
inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");

// Run the model.
let mut prediction = replicate.predictions.create(version, inputs);

println!("{}", prediction.status)
// 'starting'


prediction.reload()
println!("{}", prediction.status)
// 'processing'

println!("{}", prediction.logs)
// Some("Using seed: 3599
//      0%|          | 0/50 [00:00<?, ?it/s]
//      4%|▍         | 2/50 [00:00<00:04, 10.00it/s]
//      8%|▊         | 4/50 [00:00<00:03, 11.56it/s]
//    ")

prediction.wait()

println!("{}", prediction.status)
// 'succeeded'


match prediction.wait() {
    Ok(result) => println!("Success : {:?}", result.output),
    Err(e) => println!("Error : {}", e),
}
// Success : Some(Array [String("https://pbxt.replicate.delivery/QLDGe2rXuIQ9ByMViQEXrYCkKfDi9I3YWAzPwWsDZWMXeN7iA/out-0.png")])
```


## Cancel a prediction

You can cancel a running prediction:

```rust

let replicate = Replicate::new();

// Construct the inputs.
let mut inputs = std::collections::HashMap::new();
inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

let version = String::from("stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478");

// Run the model.
let mut prediction = replicate.predictions.create(version, inputs);

println!("{}", prediction.status)
// 'starting'

prediction.cancel()

prediction.reload()

println!("{}", prediction.status)
// 'cancelled'
```

## List predictions

You can list all the predictions you've run:

```rust
match replicate.predictions.list() {
    Ok(result) => println!("Success : {:?}", result),
    Err(e) => println!("Error : {}", e),
}
// Success : ListPredictions { ... }
```



[crates.io]: https://crates.io/crates/replicate-rust
[Latest Version]: https://img.shields.io/crates/v/replicate-rust.svg
[Documentation]: https://docs.rs/replicate-rust/badge.svg
[docs.rs]: https://docs.rs/replicate-rust
[License]: https://img.shields.io/crates/l/replicate-rust.svg
