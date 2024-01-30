use serde::{de, Serialize, Deserialize};
use serde_with::{serde_as, BoolFromInt};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalogSeq {
    pub amplitude   : Vec<f64>,
    pub times       : Vec<f64>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DigitalSeq {
    #[serde(deserialize_with = "deserialize_bool")]
    pub value       : Vec<bool>,
    pub times       : Vec<f64>,
}   
#[derive(Serialize, Deserialize, Debug)]
pub struct RS485Seq {
    #[serde(deserialize_with = "deserialize_str")]
    pub command     : Vec<String>,
    pub times       : Vec<f64>,
}     
#[derive(Serialize, Deserialize, Debug)]
pub struct VCOSeq {
    pub frequency   : Vec<f64>,
    pub times       : Vec<f64>,
}       
#[derive(Serialize, Deserialize, Debug)]
pub struct DDSSeq {
    pub amplitude : Vec<f64>,
    pub frequency : Vec<f64>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub feature_enable : Vec<bool>,
    pub feature_value  : Vec<f64>,
    pub times       : Vec<f64>,
}       
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct PulseGenSeq {
    #[serde(rename = "tDelay")]
    time_delay : f64,
    #[serde(rename = "tWidth")]
    time_width : f64,
    #[serde_as(as = "BoolFromInt")]
    polarity : bool,
}  
#[derive(Serialize, Deserialize, Debug)]
pub struct FreqFBSeq {
}    
#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceDependentData {
    Analog      (AnalogSeq      ),
    Digital     (DigitalSeq     ),
    RS485       (RS485Seq       ),
    PLLVCO      (VCOSeq         ),
    DDSRF       (DDSSeq         ),
    PulseGen    (PulseGenSeq    ),
    #[serde(rename = "FreqFeedback")]
    FreqFB      (FreqFBSeq      ),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelSequence {
    #[serde(rename = "data")]
    pub device_dependent    : DeviceDependentData,
    pub name                : String,
    #[serde(rename = "sigchan")]
    pub index_sigchan       : u8,
    pub address             : u8,
}

// How to map with result?

fn deserialize_bool<'de, D>(deserializer: D) -> Result<Vec<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let v: Vec<u8> = de::Deserialize::deserialize(deserializer)?;
    let b = v.into_iter()
                        .map(|u| !(u == 0))
                        .collect();
    Ok(b)
}

fn deserialize_str<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let v: Vec<Vec<u8>> = de::Deserialize::deserialize(deserializer)?;
    let b = v.into_iter()
                            .map_while(|bytes| String::from_utf8(bytes).ok())
                            .collect();
    Ok(b)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sequence {
    pub seq_channel : Vec<ChannelSequence>,
}

impl Sequence {
    pub fn replace(&mut self, seq : Sequence) {
        *self = seq;
    }
    pub fn update_from_json(&mut self, js : &str) {
        self.seq_channel = serde_json::from_str(js).unwrap();
    }
    pub fn into_json(&self) -> String {
        serde_json::to_string(&self.seq_channel).unwrap()
    }
    pub fn empty() -> Self {
        Sequence{ seq_channel : vec![] }
    }  
}
