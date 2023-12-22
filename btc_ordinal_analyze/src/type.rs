use serde::{Serialize};
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root<T: Serialize> {
    pub result: T,
    pub error: Value,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub txid: String,
    pub hash: String,
    pub version: i64,
    pub size: i64,
    pub vsize: i64,
    pub weight: i64,
    pub locktime: i64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
    pub blockhash: String,
    pub confirmations: i64,
    pub time: i64,
    pub blocktime: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
    pub coinbase: Option<String>,
    pub txinwitness: Option<Vec<String>>,
    pub sequence: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: f64,
    pub n: i64,
    pub script_pub_key: ScriptPubKey,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPubKey {
    pub asm: String,
    pub desc: String,
    pub hex: String,
    pub address: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockChainInfo {
    pub chain: String,
    pub blocks: i64,
    pub headers: i64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub time: i64,
    pub mediantime: i64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    #[serde(rename = "size_on_disk")]
    pub size_on_disk: i64,
    pub pruned: bool,
    pub warnings: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub hash: String,
    pub confirmations: i64,
    pub height: i64,
    pub version: i64,
    pub version_hex: String,
    pub merkleroot: String,
    pub time: i64,
    pub mediantime: i64,
    pub nonce: i64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: i64,
    pub previousblockhash: String,
    pub nextblockhash: Option<String>,
    pub strippedsize: i64,
    pub size: i64,
    pub weight: i64,
    pub tx: Vec<String>,
}