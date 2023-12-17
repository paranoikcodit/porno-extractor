use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggest {
    pub result: bool,
    pub code: i64,
    pub keywords: Vec<Keyword>,
    pub pornstar: Vec<Pornstar>,
    pub channel: Vec<Channel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
    #[serde(rename = "N")]
    pub n: String,
    #[serde(rename = "R")]
    pub r: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pornstar {
    #[serde(rename = "N")]
    pub n: String,
    #[serde(rename = "F")]
    pub f: String,
    #[serde(rename = "T")]
    pub t: String,
    #[serde(rename = "MV")]
    pub mv: i64,
    #[serde(rename = "P")]
    pub p: String,
    #[serde(rename = "RF")]
    pub rf: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(rename = "N")]
    pub n: String,
    #[serde(rename = "F")]
    pub f: String,
    #[serde(rename = "T")]
    pub t: String,
    #[serde(rename = "CPV")]
    pub cpv: bool,
    #[serde(rename = "P")]
    pub p: String,
    #[serde(rename = "RF")]
    pub rf: String,
}
