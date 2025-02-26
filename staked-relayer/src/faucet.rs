use crate::error::Error;
use hex::FromHex;
use jsonrpc_core::Value;
use jsonrpc_core_client::{transports::http as jsonrpc_http, TypedClient};
use parity_scale_codec::{Decode, Encode};
use runtime::{AccountId, PolkaBtcProvider, UtilFuncs};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
struct RawBytes(#[serde(deserialize_with = "hex_to_buffer")] Vec<u8>);

pub fn hex_to_buffer<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| Vec::from_hex(&string[2..]).map_err(|err| Error::custom(err.to_string())))
}

#[derive(Encode, Decode, Debug, Clone, serde::Serialize)]
struct FundAccountJsonRpcRequest {
    pub account_id: AccountId,
}

async fn get_funding(faucet_connection: TypedClient, staked_relayer_id: AccountId) -> Result<(), Error> {
    let funding_request = FundAccountJsonRpcRequest {
        account_id: staked_relayer_id,
    };
    let eq = format!("0x{}", hex::encode(funding_request.encode()));
    faucet_connection
        .call_method::<Vec<String>, Value>("fund_account", "", vec![eq.clone()])
        .await?;
    Ok(())
}

pub async fn connect_and_fund(provider: &PolkaBtcProvider, faucet_url: &str) -> Result<(), Error> {
    let connection = jsonrpc_http::connect::<TypedClient>(faucet_url).await?;
    // Receive user allowance from faucet
    get_funding(connection.clone(), provider.get_account_id().clone()).await?;
    Ok(())
}
