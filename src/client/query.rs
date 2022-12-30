use cosmrs::rpc::{endpoint::abci_query::AbciQuery as QueryResponse, Client};
use prost::Message;

use crate::{account::Account, CodeHash, CodeId, Contract, Error, Result};

use super::types::AccountInfo;

impl super::Client {
    pub async fn query_uscrt_balance(&self, wallet: &str) -> Result<cosmwasm_std::Uint128> {
        use cosmrs::proto::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryBalanceResponse};
        let path = "/cosmos.bank.v1beta1.Query/Balance";
        let msg = QueryBalanceRequest {
            address: wallet.to_string(),
            denom: "uscrt".to_owned(),
        };
        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryBalanceResponse>)
            .and_then(|res| match res.balance {
                Some(coin) => Ok(coin.amount.as_str().try_into()?),
                None => Ok(cosmwasm_std::Uint128::zero()),
            })
    }

    pub async fn query_code_hash_by_code_id(&self, code_id: CodeId) -> Result<CodeHash> {
        use cosmrs::proto::cosmwasm::secret::compute::v1beta1::{
            QueryCodeRequest, QueryCodeResponse,
        };
        let path = "/secret.compute.v1beta1.Query/Code";
        let msg = QueryCodeRequest {
            code_id: code_id.into(),
        };
        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryCodeResponse>)
            .and_then(|res| res.code_info.ok_or(Error::ContractInfoNotFound(code_id)))
            .map(|ci| CodeHash::from(ci.data_hash))
    }

    pub async fn query_contract<M, R>(
        &self,
        msg: &M,
        contract: &Contract,
        from: &Account,
    ) -> Result<R>
    where
        M: serde::Serialize,
        R: serde::de::DeserializeOwned,
    {
        use cosmrs::proto::cosmwasm::secret::compute::v1beta1::{
            QuerySmartContractStateRequest, QuerySmartContractStateResponse,
        };
        let path = "/secret.compute.v1beta1.Query/QuerySecretContract";
        let (nonce, encrypted) = self.encrypt_msg(msg, &contract.code_hash(), from).await?;
        let msg = QuerySmartContractStateRequest {
            address: contract.id().to_string(),
            query_data: encrypted,
        };

        let decrypter = self.decrypter(&nonce, from).await?;

        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QuerySmartContractStateResponse>)
            .and_then(|res| decrypter.decrypt(&res.data).map_err(crate::Error::from))
            .and_then(|plt| String::from_utf8(plt).map_err(crate::Error::from))
            .and_then(|b46| base64::decode(b46).map_err(crate::Error::from))
            .and_then(|buf| serde_json::from_slice(&buf).map_err(crate::Error::from))
    }

    pub(crate) async fn query_account_info(&self, account: &Account) -> Result<AccountInfo> {
        use cosmrs::proto::cosmos::auth::v1beta1::{
            BaseAccount, QueryAccountRequest, QueryAccountResponse,
        };
        let path = "/cosmos.auth.v1beta1.Query/Account";
        let msg = QueryAccountRequest {
            address: account.id().to_string(),
        };
        self.query_with_msg(path, msg)
            .await
            .and_then(try_decode_response::<QueryAccountResponse>)
            .and_then(|res| {
                res.account
                    .ok_or_else(|| Error::AccountNotFound(account.human_address()))
            })
            .and_then(try_decode_any::<BaseAccount>)
            .map(AccountInfo::from)
    }

    pub(crate) async fn query_tx_key(&self) -> Result<Vec<u8>> {
        use cosmrs::proto::cosmwasm::secret::registration::v1beta1::Key;
        let path = "/secret.registration.v1beta1.Query/TxKey";
        self.query_path(path)
            .await
            .and_then(try_decode_response::<Key>)
            .map(|key| key.key)
    }

    pub(crate) async fn query_contract_label_exists(&self, label: &str) -> Result<bool> {
        let path = format!("custom/compute/label/{label}");
        self.query_path(&path).await.map(|res| res.code.is_ok())
    }

    async fn query_with_msg(&self, path: &str, msg: impl Message) -> Result<QueryResponse> {
        self.query(path, msg.encode_to_vec()).await
    }

    async fn query_path(&self, path: &str) -> Result<QueryResponse> {
        self.query(path, vec![]).await
    }

    async fn query(&self, path: &str, data: Vec<u8>) -> Result<QueryResponse> {
        let path = path.parse().expect("abci_query path conversion failed");
        Ok(self.rpc.abci_query(Some(path), data, None, false).await?)
    }
}

fn try_decode_response<T: Message + Default>(response: QueryResponse) -> Result<T> {
    if response.code.is_err() {
        return Err(Error::AbciQuery(response.log.to_string()));
    }

    try_decode_bytes(&response.value)
}

fn try_decode_any<T: Message + Default>(any: cosmrs::Any) -> Result<T> {
    try_decode_bytes(&any.value)
}

fn try_decode_bytes<T: Message + Default>(bytes: &[u8]) -> Result<T> {
    let t = T::decode(bytes)?;
    Ok(t)
}
