pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Movie {
    pub id: MovieId,
    pub title: String,
    /// The rating scale out of ten stars
    pub rating: f64,
}
