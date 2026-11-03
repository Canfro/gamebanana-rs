use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cubeatar {
    #[serde(rename = "Speed")]
    pub speed: u64,

    #[serde(rename = "SpinAxes")]
    pub spin_axes: Vec<u64>,
}
