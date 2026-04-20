use serde::{de::DeserializeOwned};

#[derive(Clone)]
pub struct GameInfo {
	pub name: String,
	pub description: String,
	pub url: String,

	pub id: String,

	pub price: i32,
	pub price_fmt: String,
	pub currency_code: String,
}

pub struct GameSource<'a> {
	pub url: &'a str,
	pub origin: &'a str,
	pub referer: &'a str,
}

#[allow(dead_code)]
pub trait GameDataset : DeserializeOwned {
	fn get_all_games(&self) -> Vec<GameInfo>;

	fn get_free_games(&self) -> Vec<GameInfo> {
		let games = self.get_all_games();

		games
			.iter()
			.filter(|it| it.price <= 0)
			.map(|it| it.clone())
			.collect()
	}
	
	fn request_data() -> Option<Self> {
		let source = Self::get_source();

		let Ok(response) = minreq::get(source.url)
			.with_header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
			.with_header("Accept", "application/json, text/plain, */*")
			.with_header("Accept-Language", "en-US,en;q=0.9")
			.with_header("Referer", source.referer)
			.with_header("Origin", source.origin)
			.send() else { return None; };

		let Ok(body) = response.as_str() else { return None; };
		let Ok(data) = serde_json::from_str::<Self>(body) else { return None; };

		Some(data)
	}

	fn request_all_games() -> Vec<GameInfo> {
		Self::request_data()
			.map(|d| d.get_all_games())
			.unwrap_or_default()
	}

	fn request_free_games() -> Vec<GameInfo> {
		Self::request_data()
			.map(|d| d.get_free_games())
			.unwrap_or_default()
	}

	fn get_source<'a>() -> GameSource<'a>;
}