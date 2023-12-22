mod r#type;

//extern crate reqwest;
extern crate serde_json;

use std::error::Error;
use std::str::FromStr;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tokio::runtime::Runtime;

extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, RpcApi};


use hyper::http::Uri;
use hyper_tls::HttpsConnector;
use tokio::net::{lookup_host, TcpStream};
use hyper::client::HttpConnector;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use crate::r#type::{Block, BlockChainInfo, Root, Transaction};


const USER: &str = "";
const PASSWORD: &str = "";
const RPC_URL: &str = "https://soft-tiniest-sea.btc.quiknode.pro/6fa35118f891d106cd90dd551594ddd703e6e9d4/";

#[derive(Debug, Serialize, Deserialize)]
struct Msg {
    msg_type: String,
    content: Text,
}

#[derive(Debug, Serialize, Deserialize)]
struct Text {
    text: String,
}

async fn notify_lark(pushed_msg: String) -> Result<(), Box<dyn std::error::Error>> {
    //println!("increase_ratio {},increase_volume {}",increase_price,increase_volume);
    let data = Msg {
        msg_type: "text".to_string(),
        content: Text { text: pushed_msg },
    };
    let client = reqwest::Client::new();
    let res = client
        .post(
            "https://open.feishu.cn/open-apis/bot/v2/hook/a1ced4e9-b4d3-433d-82b3-b127a8cefe1c",
        )
        .json(&data)
        .header("Content-type", "application/json")
        .header("charset", "utf-8")
        .send()
        .await?;
    //send to lark
    println!("{:#?}", res.status());
    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct RpcRequest {
    method: String,
    params: Vec<String>,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcResponse {
    result: serde_json::Value,
    error: Option<serde_json::Value>,
    id: Option<u64>,
}

async fn get_chain_info(client: &reqwest::Client) -> Result<BlockChainInfo, Box<dyn Error>> {
    let res = client.post(RPC_URL)
        .basic_auth(USER, Some(PASSWORD))
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "rustclient",
            "method": "getblockchaininfo",
            "params": []
        }))
        .send()
        .await?;

    let response_text = res.text().await?;

    let chain_info: Root<BlockChainInfo> = serde_json::from_str(&response_text).unwrap();
    Ok(chain_info.result)
}

async fn get_block(client: &reqwest::Client, block_hash: &str) -> Result<Block, Box<dyn Error>> {
    let res = client.post(RPC_URL)
        .basic_auth(USER, Some(PASSWORD))
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "rustclient",
            "method": "getblock",
            "params": [block_hash,true]
        }))
        .send()
        .await?;

    let response_text = res.text().await?;
    println!("response_text {}", response_text);
    let chain_info: Root<Block> = serde_json::from_str(&response_text).unwrap();
    Ok(chain_info.result)
}

async fn get_transaction(client: &reqwest::Client, txid: &str) -> Result<Transaction, Box<dyn Error>> {
    let res = client.post(RPC_URL)
        .basic_auth(USER, Some(PASSWORD))
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "rustclient",
            "method": "getrawtransaction",
            "params": [txid,true]
        }))
        .send()
        .await?;

    let response_text = res.text().await?;
    //println!("response_text {}",response_text);
    let chain_info: Root<Transaction> = serde_json::from_str(&response_text).unwrap();
    Ok(chain_info.result)
}

async fn decode_ordinal_data() -> Result<usize, Box<dyn Error>> {
    Ok(0usize)
}
/***
struct Brc20Info{
    raw_info: String, //json
    deploy_date: String,
    deployer: String,
    current_mint: usize,
    total_supply:usize,
    mint_in_1m: usize,
    mint_in_10m:usize,  //10m
    mint_in_1h:usize, //1h
    mint_in_4h:usize, //4h
    mint_in_12h:usize,
    mint_in_1d:usize,
    is_over:bool
}



async fn get_all_tx() -> Result<(), Box<dyn Error>> {
    let rpc_url = ""; // 示例 URL


    let client = reqwest::Client::new();
    let res = client.post(rpc_url)
        .basic_auth(rpc_user, Some(password))
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "rustclient",
            "method": "getblockchaininfo",
            "params": []
        }))
        .send()
        .await?;

    let response_text = res.text().await?;
    println!("Response: {}", response_text);
    Ok(())
}


 */
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut chain_info = get_chain_info(&client).await.unwrap();
    println!("{:?}", chain_info);
    let mut current_height = chain_info.blocks;
    let test1 = format!("btc {}", current_height);
    notify_lark(test1).await.unwrap();
    let empty_witness = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

    loop {
        let block = get_block(&client, &chain_info.bestblockhash).await.unwrap();
        println!("start check {}'s {} transaction", block.hash, block.n_tx);
        for (index,tx) in block.tx.iter().enumerate() {
            //cann't batch query
            //let tx = "8d3e5ad635f79327679f22bf3886502365eec5ca49c562a60f9ca01b5e150f93";
            let transaction = get_transaction(&client, &tx).await.unwrap();

            for item in transaction.vin {
                if item.txinwitness.is_none()
                    || item.txinwitness.clone().unwrap().len() != 3
                    || item.txinwitness.clone().unwrap()[1].len() <= 116
                {
                    continue;
                }

                let inscription_with_code = item.txinwitness.unwrap()[1].clone();
                let inscription: Vec<&str> = inscription_with_code.split("746578742f706c61696e3b636861727365743d7574662d3800").collect();
                if inscription.len() == 1 {
                    continue;
                }
                println!("{:?}", inscription);
                let inscription = hex::decode(inscription.last().unwrap()).unwrap();

                let data = String::from_utf8(inscription).expect("Invalid UTF-8 sequence");
                let data = format!("find a brc20 transaction data {}, at block index {} txid {}", data, index,tx);
                notify_lark(data).await.unwrap();
                println!("{}",data);
            }
        }
        while current_height == get_chain_info(&client).await.unwrap().blocks {
            println!("sleep wait for new block");
            thread::sleep(Duration::from_secs(60));
        }
    }
    Ok(())
}