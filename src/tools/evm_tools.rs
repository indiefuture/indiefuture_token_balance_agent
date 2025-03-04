
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
use crate::types::domains::eth_address::DomainEthAddress;
use crate::types::domains::uint256::DomainUint256;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use crate::tools::mpc_tool::GetTokenBalanceInputs;

/// ERC20 token ABI with just the balanceOf function
/*const ERC20_BALANCE_ABI: &str = r#"[
    {
        "constant": true,
        "inputs": [
            {
                "name": "_owner",
                "type": "address"
            }
        ],
        "name": "balanceOf",
        "outputs": [
            {
                "name": "balance",
                "type": "uint256"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }
]"#;*/

/// Gets token balance data for a specified token and wallet address
pub async fn get_token_balance_data(input: GetTokenBalanceInputs) -> 
 Result<Option<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
    // Default to Ethereum mainnet if chain ID not specified
    let chain_id = input.chain_id.unwrap_or(1);
    
    // Determine RPC URL based on chain ID
    /*let rpc_url = match chain_id {
        1 => "https://eth.llamarpc.com", // Ethereum Mainnet
        137 => "https://polygon-rpc.com", // Polygon
        42161 => "https://arb1.arbitrum.io/rpc", // Arbitrum
        10 => "https://mainnet.optimism.io", // Optimism
        43114 => "https://api.avax.network/ext/bc/C/rpc", // Avalanche
        56 => "https://bsc-dataseed.binance.org", // BSC
        _ => return Err("Unsupported chain ID".into()),
    };*/

    let rpc_url = std:: env::var("ALCHEMY_API_KEY").unwrap(); 
    
    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);





    println!("fetch 1 ");
    
    // Parse wallet address
    let wallet_address = Address::from_str(&input.wallet_address)
        .map_err(|_| "Invalid wallet address")?;


    let input_token_address_or_symbol = input.token_address_or_symbol.clone();


    let token_address = match Address::from_str(&input.token_address_or_symbol) {


        Ok(addr) => Some(addr) ,

        Err(_e) => try_get_address_from_symbol( input_token_address_or_symbol.to_string()) ,

    };


    println!("fetch2 ");

    let Some(token_address) = token_address else {
            return Err("Invalid token address or symbol".into());
    };
    
    // Parse token address
   // let token_address = Address::from_str(&input.token_address_or_symbol)
   //     .map_err(|_| "Invalid token address")?;
    


    println!("fetch3 ");

    let token_abi = include_str!( "../abi/erc20.abi.json" );

    let abi: ethabi::Contract = serde_json::from_str( token_abi  )?;
      let contract = Contract::new(token_address, abi, client);


    // Create contract instance
  //  let contract = Contract::new(token_address, serde_json::from_str(ERC20_BALANCE_ABI)?, client);
    
    // Call balanceOf function
    let balance: U256 = contract.method::<_, U256>("balanceOf", wallet_address)?.call().await?;

      //  println!("fetch 4 ");


      // let symbol: String = contract.method::<_, String>("symbol", wallet_address).map (|f| f.call().await).unwrap_or( "?".to_string() );

       //  println!("fetch 5 ");

       //   let decimals: u8 = contract.method::<_, u8>("decimals", wallet_address)?.call().await.unwrap_or( 18 );
    
   
     //println!("fetch 6 ");

    
    // Calculate balance with decimals
  //  let divisor = U256::from(10).pow(U256::from(decimals));
   // let whole_part = balance / divisor;
  //  let fractional_part = balance % divisor;
    
    // Format fractional part with leading zeros
   // let mut fractional_str = fractional_part.to_string();
  //  let padding = usize::from(decimals) - fractional_str.len();
   // let fractional_str = format!("{}{}", "0".repeat(padding), fractional_str);
    
    // Format full decimal representation
  //  let formatted_balance = format!("{}.{}", whole_part, fractional_str);
    
    // Prepare result
    let result = json!({
        "token_address": format!("{:?}", token_address),
        "wallet_address": format!("{:?}", wallet_address),
        "chain_id": chain_id,
        "raw_balance": balance.to_string(),
      //  "formatted_balance": formatted_balance,
       // "symbol": symbol,
      //  "decimals": decimals
    });
    
    Ok(Some(result))
}


 
 type TokenAddressMap =  HashMap< String,String  > ;





fn try_get_address_from_symbol(input: String ) -> Option< Address >{


    let token_address_map_str = include_str!("../config/tokens.json");
    let token_address_map : TokenAddressMap= serde_json::from_str(token_address_map_str).ok() ?;

    println!("try get {:?}",  input);
    token_address_map.get(  &input  ).map( |x|  Address::from_str( x  ).ok()   ).flatten() 


       

}