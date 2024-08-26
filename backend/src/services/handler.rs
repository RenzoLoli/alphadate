pub trait ServiceHandlerTrait<H, T> {
    async fn handle(&self, input: H) -> Result<T, String>;
}
