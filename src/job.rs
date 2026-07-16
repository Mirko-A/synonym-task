pub trait Job {
    type Input;
    type Output;
    type Error;

    fn run(&self, i: Self::Input)
    -> impl Future<Output = Result<Self::Output, Self::Error>> + Send;
}
