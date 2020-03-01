mod models;
mod client;

#[cfg(test)]
extern crate tokio;

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[tokio::test]
    async fn search_client() {
        let client = crate::client::Client::new();
        let res = client.search_word(String::from("zankoku")).await;
        println!("{:?}", res);
    }
}
