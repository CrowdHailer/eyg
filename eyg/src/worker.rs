pub trait Worker<M, S> {
    fn new() -> Self;
    fn handle(self, message: M) -> (crate::envelope::Mail<S>, Self);
}
