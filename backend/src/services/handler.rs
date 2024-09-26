pub trait ServiceHandlerTrait<H, T>
where
    H: std::fmt::Debug,
    T: std::fmt::Debug,
{
    async fn _handle(&self, input: H) -> Result<T, String>;
    fn handle(&self, input: H) -> impl std::future::Future<Output = Result<T, String>> {
        log::debug!("input -> {:#?}", input);
        async move {
            self._handle(input)
                .await
                .inspect(|_| log::debug!("Handled!"))
                .inspect_err(|err| log::error!("Error -> {}", err))
        }
    }
}
