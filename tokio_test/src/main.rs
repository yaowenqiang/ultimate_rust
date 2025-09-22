#[cfg(test)]

async fn double(n: u32) -> u32 {
    n * 2
}

mod test {
    use super::*;
    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn will_compile() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn tokio_test() {
        assert_eq!(double(2).await, 4);
    }
}
