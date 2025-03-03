
 
use crate::types::domains::bytes::DomainBytes;
use crate::types::domains::eth_address::DomainEthAddress;
use serde::Serialize;
use serde::Deserialize;
 
use chrono::{DateTime, Utc};
use ethers::types::U64;




 
#[derive( Serialize, Deserialize,  Clone,Debug)]
 pub struct RawTxInput {
    pub chain_id: i64,
    pub to_address: String ,
    pub input_bytes:   String   ,
    pub description: Option<String>,
    pub description_short: Option<String>,

 }






#[derive(Serialize, Deserialize, Debug, Clone )] 
pub struct RawTx {


	pub chain_id: i64 ,  
	pub to_address: DomainEthAddress ,

	pub input_bytes: DomainBytes , 
	pub description: String, 
	pub description_short: String, 

	pub created_at: DateTime<Utc> ,

}


#[cfg(test)]
mod tests {
    use crate::types::domains::bytes::DomainBytes;

    use super::*;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn test_json_deserialization() {
        let json_data = json!({
            "chain_id": 1,
            "to_address": "0x5AEDA56215b167893e80B4fE645BA6d5Bab767DE",
            "input_bytes": "0xdeadbeef", // Example raw bytes in hex
            "description": "Test Transaction",
            "description_short": "Tx Test",
            "created_at": "2024-02-20T15:30:00Z"
        });

        let raw_tx: RawTx = serde_json::from_value(json_data).expect("Failed to deserialize JSON");

        // Expected values
        let expected_chain_id = 1;
        let expected_to_address = DomainEthAddress("0x5AEDA56215b167893e80B4fE645BA6d5Bab767DE".parse().unwrap());
        let expected_raw_bytes = DomainBytes(hex::decode("deadbeef").unwrap()); // assuming Web3RawBytes wraps a Vec<u8>
        let expected_created_at = "2024-02-20T15:30:00Z".parse::<DateTime<Utc>>().unwrap();

        // Assertions
        assert_eq!(raw_tx.chain_id, expected_chain_id);
        assert_eq!(raw_tx.to_address, expected_to_address);
        assert_eq!(raw_tx.input_bytes, expected_raw_bytes); // Ensure this comparison is valid based on Web3RawBytes definition
        assert_eq!(raw_tx.description, "Test Transaction");
        assert_eq!(raw_tx.description_short, "Tx Test");
        assert_eq!(raw_tx.created_at, expected_created_at);
    }
}
