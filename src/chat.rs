use crate::methods::{fetch_messages::FetchMessages, send_message::SendMessage};

#[derive(Default, Clone)]
pub struct Chat {
	base_url: String
}

impl Chat
{
	pub fn new(base_url: &str) -> Self
	{
		Self { base_url: base_url.to_string() }
	}

	pub fn send_message(self, _params: SendMessage) -> ()
	{
		// TODO: Send HTTP Request to Flock API and return response
	}

	pub fn fetch_messages(_params: FetchMessages) -> ()
	{
		// TODO: Send HTTP Request to Flock API and return response
	}
}