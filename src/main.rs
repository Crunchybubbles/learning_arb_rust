use anyhow::Result;
use ethers::{prelude::*, utils::Ganache};
use std::convert::TryFrom;
use std::sync::Arc;

abigen!(
    FlashQuery,
    "FlashBotsUniswapQuery.json"
);

#[derive(Debug)]
struct UniswapV2Pool {
    token0: Address,
    token1: Address,
    pool_addr: Address,
    reserve0: U256,
    reserve1: U256,
}
    


#[tokio::main]
async fn main() -> Result<()>  {
    // let gport = 8555u16;
    // let ganache = Ganache::new().port(gport).mnemonic("brownies are good").fork("http://localhost:8545").spawn();
    let client = Provider::<Http>::try_from("http://localhost:8545")?;
    // connect to the network
    // let client = Provider::<Http>::try_from(ganache.endpoint())?;
    // let accounts = provider.get_accounts().await?;

    let client = Arc::new(client);

    let address = "0x5EF1009b9FCD4fec3094a5564047e190D72Bd511".parse::<Address>()?;
    let query = FlashQuery::new(address, Arc::clone(&client));
    let uni_v2_addr = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;
    let start_index = "0".parse::<U256>()?;
    let stop_index = "999".parse::<U256>()?;
    
    let pairs = query.get_pairs_by_index_range(uni_v2_addr,start_index,stop_index).call().await?;
    let mut pool_addr: Vec<Address> = Vec::new();
    for i in 0..=999 {
	pool_addr.push(pairs[i][2]);
    }
    let reserves = query.get_reserves_by_pairs(pool_addr).call().await?;
    let mut pool_data: Vec<UniswapV2Pool> = Vec::new();
    for i in 0..=999 {
	pool_data.push(UniswapV2Pool {token0: pairs[i][0], token1: pairs[i][1], pool_addr: pairs[i][2], reserve0: reserves[i][0], reserve1: reserves[i][1]});
    }
    println!("pools {:#?}", pool_data);
    
    
    Ok(())
}
