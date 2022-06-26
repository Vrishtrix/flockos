#[derive(Default, Clone)]
pub struct Channel {
	base_url: String
}

impl Channel
{
	pub fn new(base_url: &str) -> Self
	{
		Self { base_url: base_url.to_string() }
	}
}