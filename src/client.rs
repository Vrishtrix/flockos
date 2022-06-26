use crate::{chat::Chat, channel::Channel};

#[derive(Default, Clone)]
pub struct FlockClient
{
	token: String,
	chat: Chat,
	channel: Channel,
	// users: Users,
	// roster: Roster
}

impl FlockClient
{
	fn new(self, token: &str) -> Self
	{
		let base_url = String::from("https://api.flock.co/v1/");

		Self
		{
			token: token.to_string(),
			chat: Chat::new(&base_url),
			channel: Channel::new(&base_url),
			..Default::default()
		}
	}
}