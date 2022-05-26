use anyhow::{Error, Result};
use ethers::{prelude::*, utils::Ganache};
use std::convert::TryFrom;
use std::sync::Arc;
use std::collections::HashMap;
use serde::Deserialize;
// const usdc: Address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse::<Address>().unwrap();
// const wbtc: Address = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599".parse::<Address>().unwrap();


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

impl UniswapV2Pool {
    fn amount1_out(&self, amount0_in: U256) -> U256 {
	self.reserve1 * (amount0_in * 997) / (self.reserve0 * 1000 + (amount0_in * 977))
    }
    
    fn amount0_out(&self, amount1_in: U256) -> U256 {
	self.reserve0 * (amount1_in * 997) / (self.reserve1 * 1000 + (amount1_in * 977))
    }

    // fn token0_decimals(&self) -> u64 {
    // 	if self.token0 == usdc {
    // 	    return 6;
    // 	} if self.token0 == wbtc {
    // 	    return 8;
    // 	} else {
    // 	    return 18;
    // 	}
    // }
    
    // fn token1_decimals(&self) -> u64 {
    // 	if self.token0 == usdc {
    // 	    return 6;
    // 	} if self.token0 == wbtc {
    // 	    return 8;
    // 	} else {
    // 	    return 18;
    // 	}
    // }

    fn copy(&self) -> UniswapV2Pool {
	return UniswapV2Pool {
	    token0: self.token0,
	    token1: self.token1,
	    pool_addr: self.pool_addr,
	    reserve0: self.reserve0,
	    reserve1: self.reserve1
	};
	
    }
}

// #[derive(Debug)]
// struct Arb {
//     token: Address,
//     profit: U256,
//     pool1: UniswapV2Pool,
//     pool2: UniswapV2Pool,
// }


// // fn find_loops(pools: Vec<UniswapV2Pool>) -> Vec<Vec<UniswapV2Pool>> {
// //     let mut pool_loops: Vec<Vec<UniswapV2Pool>> = Vec::new();
// //     for i in 0..pools.len() {
// // 	let mut pool_loop: Vec<UniswapV2Pool> = Vec::new();
// // 	for j in 0..pools.len() {
// // 	    if pools[i].pool_addr != pools[j].pool_addr && pools[i].token0 == pools[j].token0 || pools[i].token1 == pools[j].token0 || pools[i].token0 == pools[j].token1 || pools[i].token1 == pools[j].token1 {

// // 		    pool_loop.push(pools[i].copy());
// // 		    pool_loop.push(pools[j].copy());
		
// // 	    }
// // 	}
// // 	if pool_loop.len() != 0 {
// // 	    pool_loops.push(pool_loop);
// // 	}
// //     }
// //     return pool_loops;
// // }

// fn find_loops(pools: Vec<UniswapV2Pool>) -> Vec<Vec<UniswapV2Pool>> {
//     let mut pool_loops: Vec<Vec<UniswapV2Pool>> = Vec::new();
//     for i in 0..pools.len() {
// 	for j in 0..pools.len() {
// 	if pools[i].pool_addr != pools[j].pool_addr &&
// 		pools[i].token0 == pools[j].token0 ||
// 		pools[i].token1 == pools[j].token0 ||
// 		pools[i].token0 == pools[j].token1 ||
// 		pools[i].token1 == pools[j].token1
// 	    {
// 		for k in 0..pools.len() {
// 		    if pools[k].pool_addr != pools[j].pool_addr &&
// 			pools[k].pool_addr != pools[i].pool_addr &&
// 			pools[k].token0 == pools[j].token0 ||	
// 	   		pools[k].token1 == pools[j].token0 ||
// 			pools[k].token0 == pools[j].token1 ||
// 			pools[k].token1 == pools[j].token1
// 		    {
// 			if pools[k].token0 == pools[i].token0 ||
// 			    pools[k].token0 == pools[i].token1 ||
// 			    pools[k].token1 == pools[i].token0 ||
// 			    pools[k].token1 == pools[i].token1 
// 			{
// 			    let mut pool_loop: Vec<UniswapV2Pool> = Vec::new();
// 			    pool_loop.push(pools[i].copy());
// 			    pool_loop.push(pools[j].copy());
// 			    pool_loop.push(pools[k].copy());
// 			    pool_loops.push(pool_loop);
// 			}
			    
// 		    }
// 		}
// 	    }
// 	}

//     }
//     return pool_loops;
// }



// fn flashable() -> Result<Vec<Address>, anyhow::Error > {
//     let addrs = ["0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", "0x6B175474E89094C44Da98b954EedeAC495271d0F", "0x0bc529c00c6401aef6d220be8c6ea1667f6ad93e", "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xE41d2489571d322189246DaFA5ebDe1F4699F498", "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984", "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", "0x0D8775F648430679A709E98d2b0Cb6250d2887EF", "0x4Fabb145d64652a948d72533023f6E7A623C7C53", "0xF629cBd94d3791C9250152BD8dfBDF
// 380E2a3B9c", "0xdd974D5C2e2928deA5F71b9825b8b646686BD200", "0x514910771AF9Ca656af840dff83E8264EcF986CA", "0x0F5D2fB29fb7d3CFeE444a200298f468908cC942", "0x9f8F72aA9304c8B593d555F12eF6589cC3A579A2", "0x408e41876cCCDC0F92210600ef50372656052a38", "0xC011a73ee8576Fb46F5E1c5751cA3B9Fe0af2a6F", "0x57Ab1ec28D129707052df4dF418D58a2D46d5f51", "0x0000000000085d4780B73119b644AE5ecd22b376", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "0xD533a949740bb3306d119CC777fa900bA034cd52", "0x056Fd409E1d7A124BD7017459dFEa2F387b6d5Cd", "0xba100000625a3754423978a60c9317c58a424e3D", "0x8798249c2E607446EfB7Ad49eC89dD1865Ff4272", "0xD5147bc8e386d91Cc5DBE72099DAC6C9b99276F5", "0x03ab458634910AaD20eF5f1C8ee96F1D6ac54919"];
//     let mut flash_token: Vec<Address> = Vec::new(); 
//     for addr in addrs {
// 	flash_token.push(addr.parse::<Address>()?);
//     }
//     Ok(flash_token)
// }

fn markets_of_token(token: Address, market_data: &Vec<UniswapV2Pool>) -> Vec<UniswapV2Pool> {
    let mut markets: Vec<UniswapV2Pool> = Vec::new();
    for i in 0..market_data.len() {
	if (market_data[i].token0 == token || market_data[i].token1 == token) {//&& (market_data[i].reserve0 > one_eth || market_data[i].reserve1 > one_eth) { 
	    let d: UniswapV2Pool = market_data[i].copy();
	    markets.push(d) 
	}
    }
    return markets
}

// // fn swap_stuff(market_data: &Vec<UniswapV2Pool>) -> Result<Vec<Arb>, anyhow::Error> {
// //     let flash_tokens = flashable()?;
// //     let big_zero = "0".parse::<U256>()?;
// //     let initial_amount = "1000000000000000000".parse::<U256>()?;
// //     let mut arbs: Vec<Arb> = Vec::new();
// //     let zero_addr = "0x0000000000000000000000000000000000000000".parse::<Address>()?;
// //     for i in 0..flash_tokens.len() {
// // 	for market1 in markets_of_token(flash_tokens[i], market_data) {
// // 	    let mut token1 = zero_addr;
// // 	    let mut amount1 = big_zero;
// // 	    if market1.token0 == flash_tokens[i] && (market1.reserve0 != big_zero && market1.reserve1 != big_zero) {
// // 		token1 = market1.token1;
// // 		amount1 = market1.amount1_out(initial_amount);
// // 	    }

// // 	    if market1.token1 == flash_tokens[i] && (market1.reserve0 != big_zero && market1.reserve1 != big_zero) {
// // 		token1 = market1.token0;
// // 		amount1 = market1.amount0_out(initial_amount);
// // 	    }

// // 	    for market2 in markets_of_token(token1, market_data) {
// // 	    let mut token2 = zero_addr;
// // 	    let mut amount2 = big_zero;
// // 		if market2.token0 == flash_tokens[i] && (market2.reserve0 != big_zero && market2.reserve1 != big_zero) {
// // 		    token2 = market2.token1;
// // 		    amount2 = market2.amount1_out(amount1);
// // 		}

// // 		if market2.token1 == flash_tokens[i] && (market2.reserve0 != big_zero && market2.reserve1 != big_zero) {
// // 		    token2 = market2.token0;
// // 		    amount2 = market2.amount0_out(amount1);
// // 		}

// // 		if token2 == flash_tokens[i] {
// // 		    let gain = amount2 - initial_amount;
// // 		    if gain > big_zero {
// // 			arbs.push(Arb {token: token2, profit: gain, pool1: market1, pool2: market2})
// // 		    }
// // 		}
// // 	    }
// // 	}
// //     }
// //     Ok(arbs)
#[derive(Debug, Deserialize)]
struct Token {
    address: String,
    chainId: u64,
    decimals: u64,
    #[serde(skip)]
    logoURI: String,
    name: String,
    symbol: String,
}


#[derive(Debug, Deserialize)]
struct CoinGecko {
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    logoURI: String,
    #[serde(skip)]
    keywords: Vec<String>,
    #[serde(skip)]
    timestamp: String,
    tokens: Vec<Token>,
    #[serde(skip)]
    version: HashMap<String, u64>,
	
}



#[tokio::main]
async fn main() -> Result<()>  {
    // let gport = 8555u16;
    // let ganache = Ganache::new().port(gport).mnemonic("brownies are good").fork("http://localhost:8545").spawn();
    let response = reqwest::Client::new().get("https://tokens.coingecko.com/uniswap/all.json").send().await?.text().await?;
    let coinlist: CoinGecko = serde_json::from_str(&response)?;
    // println!("{:#?}",coinlist);
    let client = Provider::<Http>::try_from("http://localhost:8545")?;
    // // connect to the network
    // // let client = Provider::<Http>::try_from(ganache.endpoint())?;
    // // let accounts = provider.get_accounts().await?;

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

    let mut markets: HashMap<Address, Vec<UniswapV2Pool>> = HashMap::new();
    for i in 0..coinlist.tokens.len() {
	let token = coinlist.tokens[i].address.parse::<Address>()?;
	markets.insert(token, markets_of_token(token, &pool_data));
    }
    println!("{:#?}", markets);
    // // let flash_tokens = flashable()?;
    // // let mut flash_markets = HashMap::new();
    // // for token in flash_tokens {
    // // 	flash_markets.insert(token, markets_of_token(token, &pool_data));
    // // }
    
    // // println!("{:#?}", flash_markets);
    // let mut all_tokens_markets = HashMap::new();
    // for pool in &pool_data {
    // 	if !all_tokens_markets.contains_key(&pool.token0) {
    // 	    all_tokens_markets.insert(&pool.token0, markets_of_token(pool.token0, &pool_data));
    // 	}
    // 	if !all_tokens_markets.contains_key(&pool.token1) {
    // 	    all_tokens_markets.insert(&pool.token1, markets_of_token(pool.token1, &pool_data));
    // 	}
    // }
    // let weth = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse::<Address>()?;
    //println!("{:#?}", all_tokens_markets.get(&weth));
	     
    // let arbs = swap_stuff(&pool_data);
    // println!("{:#?}", arbs);
    // let pool_loops:Vec<Vec<UniswapV2Pool>> = find_loops(pool_data);
    // for l in pool_loops {
    // 	println!("{:#?}", l.len());
    // }
    // println!("pools {:#?}", pool_data);
    // let a: u64 = 10;
    // let o1 = pool_data[0].amount1_out("1000000");
    // let o0 = pool_data[0].amount0_out();
    // println!("amount1 out {} amount0 out {}", o1, o0);
    // println!("pool 0 {:#?}", pool_data[0]);
    Ok(())
}
