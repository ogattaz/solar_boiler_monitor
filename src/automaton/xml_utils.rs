use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use html_escape::decode_html_entities;

#[derive(Debug, Serialize, Deserialize)]
pub struct VarDescriptions {
    #[serde(rename = "@statut")]
    pub status: String,
    #[serde(rename = "donnee", default)]
    pub descriptions: Vec<VarDescription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarDescription {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@nom_carte")]
    pub name: String,
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@id_statut")]
    pub id_statut: u32,
    #[serde(rename = "@droit_modification")]
    pub droit_modification: u32,
}

impl VarDescriptions {
    pub fn from_xml(xml: &str) -> Result<Self, quick_xml::DeError> {
        from_str(xml)
    }

    pub fn size(&self) -> usize {
        self.descriptions.len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSlots {
    #[serde(rename = "@statut")]
    pub status: String,
    #[serde(rename = "@message")]
    pub message: String,
    #[serde(rename = "valeur", default)]
    pub values: Vec<ValueSlot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueSlot {
    #[serde(rename = "@heure")]
    pub ticks: u64,
    #[serde(rename = "@donnee")]
    pub id: u32,
    #[serde(rename = "@valeur")]
    pub value: String,
}

impl ValueSlots {
    pub fn from_xml(xml: &str) -> Result<Self, quick_xml::DeError> {
        from_str(xml)
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn is_on_error(&self) -> bool {
        self.status == "echec"
    }

    pub fn last_ticks(&self) -> u64 {
        self.values.iter().map(|slot| slot.ticks).max().unwrap_or(0)
    }

    pub fn dump_values(&self) -> String {
        self.values
            .iter()
            .map(|slot| format!("{:5}=[{:<30}]", slot.id, slot.value))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn decode_value(encoded_value: &str) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(encoded_value).unwrap();
    let decoded_str = String::from_utf8(decoded_bytes).unwrap();
    decode_html_entities(&decoded_str).to_string()
}

pub fn encode_value(value: &str) -> String {
    general_purpose::STANDARD.encode(value)
}
