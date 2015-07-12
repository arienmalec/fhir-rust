pub trait TryFrom<T> {
	type Error;
	fn try_from(v: T) -> Result<Self,Self::Error>;
}