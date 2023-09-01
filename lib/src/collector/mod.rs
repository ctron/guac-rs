use serde::{Deserialize, Serialize};

#[allow(clippy::module_inception)]
pub mod collector;
pub mod emitter;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub blob: Vec<u8>,
    pub r#type: DocumentType,
    pub format: FormatType,
    pub encoding: EncodingType,
    pub source_information: SourceInformation,
}

#[derive(Serialize, Deserialize)]
pub enum DocumentType {
    SLSA,
    ITE6,
    ITE6VUL,
    DSSE,
    SPDX,
    JsonLines,
    SCORECARD,
    CyclonDX,
    DepsDev,
    UNKNOWN,
}

#[derive(Serialize, Deserialize)]
pub enum FormatType {
    JSON,
    JsonLines,
    XML,
    UNKNOWN,
}

#[derive(Serialize, Deserialize)]
pub enum EncodingType {
    BZIP2,
    UNKNOWN,
}

#[derive(Serialize, Deserialize)]
pub struct SourceInformation {
    pub collector: String,
    pub source: String,
}
