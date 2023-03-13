use ethers::core::types::Chain;

pub fn get_chain_rpc_urls(chain: Chain) -> Vec<String> {
    let rpcs = match chain {
        Chain::Arbitrum => vec!["https://arb1.arbitrum.io/rpc"],
        Chain::ArbitrumGoerli => vec!["https://goerli-rollup.arbitrum.io/rpc"],
        Chain::Avalanche => vec!["https://api.avax.network/ext/bc/C/rpc"],
        Chain::AvalancheFuji => vec!["https://api.avax-test.network/ext/bc/C/rpc"],
        _ => unimplemented!(),
    };
    rpcs.into_iter()
        .map(|item| item.to_owned())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_chain_rpc_urls() {}
}
