// #[derive(Clone)]
pub struct Model {
    // Holds a reference to a Replicate
    pub parent: crate::client::Client,
    pub versions : crate::structs::Version::Version,
}