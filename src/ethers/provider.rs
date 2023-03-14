use super::chain::get_chain_rpc_urls;
use ethers::{
    providers::{Http, Middleware, Provider, ProviderError},
    types::{BlockNumber, Chain},
};
use futures::stream;
use futures::stream::StreamExt;
use getset::Getters;
use std::sync::Arc;

pub type HttpProvider = Provider<Http>;
pub type SharedHttpProvider = Arc<HttpProvider>;

#[derive(Debug, Clone, Getters)]
#[allow(dead_code)]
pub struct HttpProviderHandler {
    #[getset(get)]
    chain: Chain,
    #[getset(get)]
    rpc_urls: Vec<String>,
}

impl HttpProviderHandler {
    pub fn new(chain: Chain) -> Result<Self, ProviderError> {
        let rpc_urls = get_chain_rpc_urls(chain);
        let valid_rpc_urls = rpc_urls
            .clone()
            .into_iter()
            .filter(|url| HttpProvider::try_from(url.as_str()).map_or(false, |_| true))
            .collect::<Vec<String>>();
        if valid_rpc_urls.len() == 0 {
            return Err(ProviderError::UnsupportedRPC);
        }
        Ok(HttpProviderHandler {
            chain,
            rpc_urls: valid_rpc_urls,
        })
    }

    pub fn get_list_providers(&self) -> Vec<HttpProvider> {
        self.rpc_urls
            .clone()
            .into_iter()
            .filter_map(|url| HttpProvider::try_from(url).map_or(None, |provider| Some(provider)))
            .collect::<Vec<HttpProvider>>()
    }

    /// Fetching provider which has fastest response to get_block call
    pub async fn fetch_best_provider(&self) -> Result<HttpProvider, ProviderError> {
        let providers = self.get_list_providers();
        let tasks = providers.into_iter().map(|p| {
            tokio::spawn(async move {
                let block = p.get_block(BlockNumber::Latest).await.unwrap_or(None);
                block.map_or(None, |_| Some(p))
            })
        });
        let mut tasks_stream = stream::iter(tasks).buffer_unordered(10);

        while let Some(task_result) = tasks_stream.next().await {
            if let Some(provider) = task_result.unwrap_or(None) {
                return Ok(provider);
            } else {
                continue;
            }
        }
        Err(ProviderError::CustomError("Provider not found".to_owned()))
    }
}

#[cfg(test)]
mod provider_test {
    use super::*;
    use claims::{assert_ge, assert_ok};

    fn setup_http_provider_handler() -> HttpProviderHandler {
        assert_ok!(HttpProviderHandler::new(Chain::AvalancheFuji))
    }

    #[test]
    fn test_get_list_provider() {
        let http_provider_handler = setup_http_provider_handler();
        let providers = http_provider_handler.get_list_providers();
        assert_ge!(providers.len(), 1);
    }

    #[tokio::test]
    async fn test_fetch_best_provider() {
        let http_provider_handler = setup_http_provider_handler();
        let best_provider = assert_ok!(http_provider_handler.fetch_best_provider().await);
        println!("best provider url {:?}", best_provider.url().as_str());
    }
}
