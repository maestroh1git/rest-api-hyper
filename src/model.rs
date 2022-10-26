use serde::{Deserialize, Serialize};

/// Represents a car
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Car {
    /// A unique identifier for a car record
    pub id: String,

    /// brand name
    pub brand: String,

    /// model 
    pub model: String,

    /// year 
    pub year: u16
  
}