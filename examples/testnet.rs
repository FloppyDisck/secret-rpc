use secret_rpc::Contract;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    TokenInfo {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: Option<cosmwasm_std::Uint128>,
    },
}

#[tokio::main]
async fn main() {
    let client = secret_rpc::SecretRPC::new()
        .host("http://rpc.testnet.secretsaturn.net")
        .enclave_key("e24a22b31e3d34e0e00bcd32189548f1ccbdc9cda8f5a266219b908582b6f03f")
        .connect()
        .unwrap();

    let contract = Contract::try_from_address_with_code_hash(
        "secret18q8y7ulptuznz70ng7w7yt822ftt7g93v0lvwt",
        "5266a630e2b8ef910fb2515e1d3b5be95d4bd48358732788d8fcd984ee966bc1",
    )
    .unwrap();

    let ans: QueryAnswer = client
        .query_contract(&QueryMsg::TokenInfo {}, &contract, &secret_rpc::a())
        .await
        .unwrap();

    println!("{ans:#?}");
}
