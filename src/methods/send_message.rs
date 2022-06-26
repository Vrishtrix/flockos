use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SendAs {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SendMessage
{
	to: String,
	text: String,
	flockml: Option<String>,
	notification: Option<String>,
	mentions: Option<Vec<String>>,
	send_as: Option<SendAs>,
	attachments: Option<Vec<String>>,
	on_behalf_of: Option<String>,
	visible_to: Option<Vec<String>>
}

impl SendMessage
{
	pub fn new(to: &str, text: &str) -> Self
	{
		Self 
		{ 
			to: to.to_string(),
			text: text.to_string(),
			..Default::default()
		}
	}

	pub fn set_flockml(mut self, flockml: &str) -> Self
	{
		self.flockml = Some(flockml.to_string());
		self
	}

	pub fn set_notification(mut self, notif: &str) -> Self
	{
		self.notification = Some(notif.to_string());
		self
	}
	pub fn set_mentions(mut self, mentions: Vec<String>) -> Self
	{
		self.mentions = Some(mentions);
		self
	}

	pub fn set_send_as(mut self, sa: SendAs) -> Self
	{
		self.send_as = Some(sa);
		self
	}

	pub fn set_attachments(mut self, attachments: Vec<String>) -> Self
	{
		self.attachments = Some(attachments);
		self
	}

	pub fn set_on_behalf_of(mut self,obh: &str) -> Self
	{
		self.on_behalf_of = Some(obh.to_string());
		self
	}

	pub fn set_visible_to(mut self, visible_to: Vec<String>) -> Self
	{
		self.visible_to = Some(visible_to);
		self
	}
}