mod inits_tests {
    use std::sync::atomic::AtomicU32;

    use crate::inits;

    #[tokio::test]
    async fn init_df() {
        let x = inits::init("count.json").await;

        assert_eq!(x.contains("CDU"), true);
        assert_eq!(x.contains("SPD"), true);
        assert_eq!(x.contains("AfD"), true);
        assert_eq!(x.contains("FDP"), true);
        assert_eq!(x.contains("B90G"), true);
    }

    #[tokio::test]
    async fn init_reset() {
        let x = inits::init("count.json").await;

        let a = inits::init("count.json").await;

        x.increment("CDU");

        inits::update("count.json", &x).await;

        let y = inits::init("count.json").await;

        assert_eq!(serde_json::to_string(&x).unwrap(), serde_json::to_string(&y).unwrap());

        let b = inits::reset_n_init("candidates", "count.json").await;

        assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap())
    }
}
