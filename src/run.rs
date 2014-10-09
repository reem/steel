pub trait Run {
    type Report;

    fn run(&self) -> <Self as Run>::Report;
}

