use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ECBResponse {
    pub header: Header,
    #[serde(rename = "dataSets")]
    pub datasets: Vec<DataSet>,
    pub structure: Structure,
}


// Header


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Header {
    pub id: String,
    pub test: bool,
    pub prepared: String,
    pub sender: Sender,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sender {
    pub id: String,
}


// Data sets


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataSet {
    pub action: String,
    #[serde(rename = "validFrom")]
    pub valid_from: String,
    pub series: HashMap<String, Series>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Series {
    pub observations: Option<HashMap<String, Vec<Option<f64>>>>,
}


// Structure


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Structure {
    pub links: Vec<Link>,
    pub name: String,
    pub dimensions: Dimension,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub title: String,
    pub rel: String,
    pub href: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub series: Vec<SeriesDimension>,
    pub observation: Option<Vec<SeriesDimensionObservation>>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesDimension {
    pub id: String,
    pub name: String,
    pub values: Vec<SeriesDimensionValue>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesDimensionValue {
    pub id: String,
    pub name: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesDimensionObservation {
    pub id: String,
    pub name: String,
    pub role: String,
    pub values: Vec<SeriesDimensionObservationValue>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeriesDimensionObservationValue {
    pub id: String,
    pub name: String,
    pub start: String,
    pub end: String,
}