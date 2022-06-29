use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FetchMessages
{
	chat: String,
	uids: Vec<String>,
}

impl FetchMessages
{
	pub fn new(chat: &str, uids: Vec<String>) -> Self
	{
		Self
		{
			chat: chat.to_string(),
			uids
		}
	}
}