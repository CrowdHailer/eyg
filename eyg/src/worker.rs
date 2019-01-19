pub trait Worker<M> {
    fn new() -> Self;
    fn handle<S>(self, message: M) -> (crate::envelope::Mail<S>, Self);
}
