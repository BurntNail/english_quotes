pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn l(obj: impl Into<A>) -> Self {
        Self::Left(obj.into())
    }
    pub fn r(obj: impl Into<B>) -> Self {
        Self::Right(obj.into())
    }
}
