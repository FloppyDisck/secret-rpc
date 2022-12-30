// Chain info
pub static CHAIN_PREFIX: &str = "secret";
pub static SCRT_DERIVATION_PATH: &str = "m/44'/529'/0'/0/0";
pub static COIN_DENOM: &str = "uscrt";

// Host info
pub static TESTNET_CHAIN_ID: &str = "pulsar-2";
pub static TESTNET_HOST: &str = "http://rpc.testnet.secretsaturn.net";
pub static TESTNET_ENCLAVE_KEY: &str =
    "e24a22b31e3d34e0e00bcd32189548f1ccbdc9cda8f5a266219b908582b6f03f";

// RPC ports
pub const DEFAULT_PORT: u16 = 26657;
pub const FAUCET_PORT: u16 = 5000;

// Gas info
pub const UPLOAD_GAS: u64 = 1_000_000;
pub const UPLOAD_AMOUNT: u64 = 250_000;
pub const INIT_GAS: u64 = 500_000;
pub const INIT_AMOUNT: u64 = 125_000;
pub const EXEC_GAS: u64 = 200_000;
pub const EXEC_AMOUNT: u64 = 50_000;
