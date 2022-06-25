use std::error::Error;

pub struct FlockClient {}

pub trait HttpClient<T>
{
	type Error;

	fn post
	(
		&self,
		url : &str,
		headers: Option<T>,
		payload: String,
	)
	-> Result<String, Self::Error>;
}

impl HttpClient<String> for FlockClient
{
	type Error = dyn Error;

	fn post
		(
			&self,
			url : &str,
			headers: Option<T>,
			payload: String,
		)
		-> Result<String, Self::Error> {
			todo!()
		}
}