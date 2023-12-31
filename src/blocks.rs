//! # Blocks
//!
//! This module contains everything needed to work with blocks, get information about them, etc.

use url::Url;

use crate::DaemonNode;

pub struct Block {}

fn is_valid_url(url_str: &str) -> bool {
    match Url::parse(url_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Tries to decode a block hash and returns a Block struct if successful
#[tokio::main]
pub async fn get_block_info(block_height: u64, node: DaemonNode) -> Block {
    let mut node_url: String = String::new();
    let protocol = if node.tls { "https://" } else { "http://" };
    node_url.push_str(format!("{}{}:{}/json_rpc", protocol, node.url, node.port).as_str());
    if !is_valid_url(&node_url) {
        panic!("URL couldn't be constructed for given DaemonNode, please check your DaemonNode");
    }
    let resp = (ureq::get(node_url.as_str()).set("Content-Type","application/json").send_json(ureq::json!({"jsonrpc":"2.0","id":"0","method":"get_block","params":{"height":block_height}}))).unwrap();
    if resp.status() != 200 {
        panic!("Couldn't get block info, please check your DaemonNode");
    }
    /*
    When the recipient is looking for his outputs, he's doing, for each TX on the blockchain:

    Get the TX public key R, outputs P and output indexes i.
    For each index, compute P' = H_s(aR||i) + B
    Compare P' == P ?
    If equal, it's a match, continue to decode the amount.
    */
    return Block {};
}
