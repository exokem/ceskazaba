
mod api;
mod epic;

use epic::{EpicGamesStoreData};

use crate::api::{GameDataset};

fn main() {
	let games = EpicGamesStoreData::request_free_games();

	for game in games {
		println!("{} ({})", game.name, game.url);
	}
}
