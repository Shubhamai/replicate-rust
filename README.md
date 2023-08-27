<div align="center">
<h1>Replicate Rust client</h1>
</div>

<p align="center">An Unofficial Rust client for <a href="https://replicate.com">Replicate</a>
<div align="center">

<!--[![codecov](https://codecov.io/gh/shubhamai/replicate-rust/branch/main/graph/badge.svg)](https://codecov.io/gh/shubhamai/replicate-rust)-->
[![Documentation]][docs.rs] [![Latest Version]][crates.io]
[![Tests](https://github.com/Shubhamai/replicate-rust/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/Shubhamai/replicate-rust/actions/workflows/tests.yml)
<a href="https://crates.io/crates/replicate-rust"><img src="https://img.shields.io/crates/d/replicate-rust"></a>
[![Rust](https://img.shields.io/badge/rust-1.72%2B-blue.svg?maxAge=3600)](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1720-2023-08-24)
</div>

<p align="center">
    <a href="https://docs.rs/replicate-rust/">Documentation</a>
    ·
    <a href="https://crates.io/crates/replicate-rust">Crate</a>
    ·
    <a href="https://github.com/shubhamai/replicate-rust/issues">Report Bug</a>
    ·
    <a href="https://github.com/shubhamai/replicate-rust/issues">Request Feature</a>

</p>

<!-- cargo-rdme start -->

An Unofficial Rust client for [Replicate](https://replicate.com). Provides a type-safe interface by deserializing API responses into Rust structs.

## Getting Started

Add `replicate_rust` to `Cargo.toml`:

```toml
[dependencies]
replicate-rust = "0.0.3"
```

Grab your token from [replicate.com/account](https://replicate.com/account) and set it as an environment variable:

```sh
export REPLICATE_API_TOKEN=<your token>
```

Here's an example using `replicate_rust` to run a model:

```rust
use replicate_rust::{config::Config, Replicate, errors::ReplicateError};

fn main() -> Result<(), ReplicateError> {
   let config = Config::default();
   // Instead of using the default config ( which reads API token from env variable), you can also set the token directly:
   // let config = Config {
   //     auth: String::from("REPLICATE_API_TOKEN"),
   //     ..Default::default()
   // };

   let replicate = Replicate::new(config);

   // Construct the inputs.
   let mut inputs = std::collections::HashMap::new();
   inputs.insert("prompt", "a  19th century portrait of a wombat gentleman");

   let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";

   // Run the model.
   let result = replicate.run(version, inputs)?;

   // Print the result.
   println!("{:?}", result.output);
   // Some(Array [String("https://pbxt.replicate.delivery/QLDGe2rXuIQ9ByMViQEXrYCkKfDi9I3YWAzPwWsDZWMXeN7iA/out-0.png")])```

   Ok(())
}
```

## Usage

See the [reference docs](https://docs.rs/replicate-rust/) for detailed API documentation.

## Examples

- Run a model in the background:
    ```rust
    // Construct the inputs.
    let mut inputs = std::collections::HashMap::new();
    inputs.insert("prompt", "a 19th century portrait of a wombat gentleman");

    let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";

    // Run the model.
    let mut prediction = replicate.predictions.create(version, inputs)?;

    println!("{:?}", prediction.status);
    // 'starting'

    prediction.reload()?;
    println!("{:?}", prediction.status);
    // 'processing'

    println!("{:?}", prediction.logs);
    // Some("Using seed: 3599
    // 0%|          | 0/50 [00:00<?, ?it/s]
    // 4%|▍         | 2/50 [00:00<00:04, 10.00it/s]
    // 8%|▊         | 4/50 [00:00<00:03, 11.56it/s]
   

    let prediction = prediction.wait()?;

    println!("{:?}", prediction.status);
    // 'succeeded'

    println!("{:?}", prediction.output);
    ```

- Cancel a prediction:
  ```rust
  // Construct the inputs.
  let mut inputs = std::collections::HashMap::new();
  inputs.insert("prompt", "a 19th century portrait of a wombat gentleman");

  let version = "stability-ai/stable-diffusion:27b93a2413e7f36cd83da926f3656280b2931564ff050bf9575f1fdf9bcd7478";

  // Run the model.
  let mut prediction = replicate.predictions.create(version, inputs)?;

  println!("{:?}", prediction.status);
  // 'starting'

  prediction.cancel()?;

  prediction.reload()?;

  println!("{:?}", prediction.status);
  // 'cancelled'
  ```

- List predictions:
  ```rust
  let predictions = replicate.predictions.list()?;
  println!("{:?}", predictions);
  // ListPredictions { ... }
  ```

- Get model Information:
  ```rust
  let model = replicate.models.get("replicate", "hello-world")?;
  println!("{:?}", model);
  // GetModel { ... }
  ```

- Get Versions List:
  ```rust
  let versions = replicate.models.versions.list("replicate", "hello-world")?;
  println!("{:?}", versions);
  // ListModelVersions { ... }
  ```

- Get Model Version Information:
  ```rust
  let model = replicate.models.versions.get("kvfrans",
  "clipdraw",
  "5797a99edc939ea0e9242d5e8c9cb3bc7d125b1eac21bda852e5cb79ede2cd9b",)?;
  println!("{:?}", model);
  // GetModelVersion { ... }
  ```

- Get Collection Information:
  ```rust
  let collection = replicate.collections.get("audio-generation")?;
  println!("{:?}", collection);
  // GetCollectionModels { ... }//!   ```
   ```

- Get Collection Lists:
  ```rust
  let collections = replicate.collections.list()?;
  println!("{:?}", collections);
  // ListCollectionModels { ... }
  ```

<!-- cargo-rdme end -->

[crates.io]: https://crates.io/crates/replicate-rust
[Latest Version]: https://img.shields.io/crates/v/replicate-rust.svg
[Documentation]: https://docs.rs/replicate-rust/badge.svg
[docs.rs]: https://docs.rs/replicate-rust
[License]: https://img.shields.io/crates/l/replicate-rust.svg
