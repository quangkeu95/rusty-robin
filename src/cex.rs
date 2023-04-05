pub mod binance;
pub mod bitfinex;
pub mod bybit;
pub mod coinbase;

#[cfg(test)]
mod test {
    use super::*;
    use claims::{assert_gt, assert_ok};

    #[tokio::test]
    async fn test_binance_get_book_ticker() {
        let market: binance::market::Market = binance::api::Binance::new(None, None);

        let result = assert_ok!(market.get_book_ticker("BTCUSDT").await);
        assert_gt!(result.bid_price, 0.into());
        assert_gt!(result.ask_price, 0.into());
    }
}
