//! Read-only badge catalog enrichment and the account's own badge settings write.
use crate::{DiscordApi, Failure};
use discord_protocol::badges;
use model::{
	Id,
	badges::{BadgeCatalog, BadgeSettings},
};
use reqwest::Method;

impl DiscordApi {
	/// Optional catalog beside a profile. Failures other than a lost session leave the
	/// profile with its own badge list.
	pub(super) async fn load_badge_catalog(
		&self,
		user: Id,
	) -> Result<Option<BadgeCatalog>, Failure> {
		match self
			.request_limited(
				Method::GET,
				&format!("/users/{user}/badges"),
				None,
				badges::MAX_CATALOG_WIRE,
			)
			.await
		{
			Ok(bytes) => Ok(badges::decode_catalog(&bytes).ok()),
			Err(failure) if failure.ends_session() => Err(failure),
			Err(_) => Ok(None),
		}
	}

	/// One explicitly requested write, without retries.
	pub(super) async fn save_badge_settings(
		&self,
		settings: BadgeSettings,
	) -> Result<BadgeSettings, Failure> {
		let body = badges::encode_settings(&settings).map_err(|_| Failure::Protocol)?;
		self.request_limited(
			Method::PATCH,
			"/users/@me/badges/settings",
			Some(body),
			64 * 1024,
		)
		.await?;
		Ok(settings)
	}
}
