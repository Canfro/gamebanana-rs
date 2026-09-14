use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Image {
    Simple {
        #[serde(rename = "_sType")]
        ty: String,

        #[serde(rename = "_sUrl")]
        url: String,
    },
    Advanced {
        #[serde(rename = "_sType")]
        ty: String,

        #[serde(rename = "_sBaseUrl")]
        base_url: String,

        #[serde(rename = "_sCaption")]
        caption: Option<String>,

        #[serde(rename = "_sFile")]
        file: String,

        #[serde(rename = "_sFile100")]
        file100: Option<String>,

        #[serde(rename = "_sFile220")]
        file220: Option<String>,

        #[serde(rename = "_sFile530")]
        file530: Option<String>,

        #[serde(rename = "_wFile100")]
        w_file100: Option<u64>,

        #[serde(rename = "_wFile220")]
        w_file220: Option<u64>,

        #[serde(rename = "_wFile530")]
        w_file530: Option<u64>,

        #[serde(rename = "_hFile100")]
        h_file100: Option<u64>,

        #[serde(rename = "_hFile220")]
        h_file220: Option<u64>,

        #[serde(rename = "_hFile530")]
        h_file530: Option<u64>,
    },
}
