use serde::Deserialize;

use super::IpfsConfig;

#[derive(Default, Deserialize, Clone, Debug)]
pub struct Environment {
    pub rpc_url: Option<String>,
    pub account_address: Option<String>,
    pub private_key: Option<String>,
    pub keystore_path: Option<String>,
    pub keystore_password: Option<String>,
    pub world_address: Option<String>,
    pub world_block: Option<u64>,
    pub max_block_range: Option<u64>,
    pub http_headers: Option<Vec<HttpHeader>>,
    pub ipfs_config: Option<IpfsConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl Environment {
    pub fn world_address(&self) -> Option<&str> {
        self.world_address.as_deref()
    }

    pub fn rpc_url(&self) -> Option<&str> {
        self.rpc_url.as_deref()
    }

    pub fn account_address(&self) -> Option<&str> {
        self.account_address.as_deref()
    }

    pub fn private_key(&self) -> Option<&str> {
        self.private_key.as_deref()
    }

    pub fn keystore_path(&self) -> Option<&str> {
        self.keystore_path.as_deref()
    }

    pub fn keystore_password(&self) -> Option<&str> {
        self.keystore_password.as_deref()
    }
}
