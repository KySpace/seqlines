use serde::{de, Serialize, Deserialize};
use serde_with::{serde_as, BoolFromInt};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct AnalogSeq {
    amplitude   : Vec<f64>,
    times       : Vec<f64>,
}
#[derive(Serialize, Deserialize, Debug)]
struct DigitalSeq {
    #[serde(deserialize_with = "deserialize_bool")]
    value       : Vec<bool>,
    times       : Vec<f64>,
}   
#[derive(Serialize, Deserialize, Debug)]
struct RS485Seq {
    #[serde(deserialize_with = "deserialize_str")]
    command     : Vec<String>,
    times       : Vec<f64>,
}     
#[derive(Serialize, Deserialize, Debug)]
struct VCOSeq {
    frequency   : Vec<f64>,
    times       : Vec<f64>,
}       
#[derive(Serialize, Deserialize, Debug)]
struct DDSSeq {
    amplitude : Vec<f64>,
    frequency : Vec<f64>,
    #[serde(deserialize_with = "deserialize_bool")]
    feature_enable : Vec<bool>,
    feature_value  : Vec<f64>,
    times       : Vec<f64>,
}       
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct PulseGenSeq {
    #[serde(rename = "tDelay")]
    time_delay : f64,
    #[serde(rename = "tWidth")]
    time_width : f64,
    #[serde_as(as = "BoolFromInt")]
    polarity : bool,
}  
#[derive(Serialize, Deserialize, Debug)]
struct FreqFBSeq {
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
    device_dependent    : DeviceDependentData,
    name                : String,
    #[serde(rename = "sigchan")]
    index_sigchan       : u8,
    address             : u8,
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
    seq_channel : Vec<ChannelSequence>,
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
