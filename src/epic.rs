
use crate::api::{GameDataset, GameInfo, GameSource};

pub use epic_struct::{EpicGamesStoreData, GameResult};

mod epic_struct {
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct EpicGamesStoreData {
		pub data: FreeGamesData
	}

	#[derive(Deserialize)]
	pub struct FreeGamesData {
		#[serde(rename = "Catalog")]
		pub catalog: Catalog
	}


	#[derive(Deserialize)]
	pub struct Catalog {
		#[serde(rename = "searchStore")]
		pub search_store: SearchStore
	}

	#[derive(Deserialize, Clone)]
	pub struct SearchStore {
		pub elements: Vec<GameResult>
	}

	#[derive(Deserialize, Clone)]
	pub struct GameResult {
		pub title: String,
		pub id: String,
		pub description: String,
		#[serde(rename = "offerType")]
		pub offer_type: String,
		#[serde(rename = "offerMappings")]
		pub offer_mappings: Option<Vec<OfferMapping>>,
		pub price: GamePrice,
		pub promotions: Option<GamePromotions>,
	}

	// ------------------------------------------------------- URL

	#[derive(Deserialize, Clone)]
	pub struct OfferMapping {
		#[serde(rename = "pageSlug")]
		pub slug: String,
		#[serde(rename = "pageType")]
		pub page: String,
	}

	// ------------------------------------------------------- PRICE

	#[derive(Deserialize, Clone)]
	pub struct GamePrice {
		#[serde(rename = "totalPrice")]
		pub total: TotalPrice
	}

	#[derive(Deserialize, Clone)]
	pub struct TotalPrice {
		#[serde(rename = "discountPrice")]
		pub discount_price: i32,
		#[serde(rename = "originalPrice")]
		pub original_price: i32,
		pub discount: i32,
		#[serde(rename = "currencyCode")]
		pub currency_code: String,
		#[serde(rename = "currencyInfo")]
		pub currency_info: CurrencyInfo,
		#[serde(rename = "fmtPrice")]
		pub fmt_price: FmtPrice,
	}

	#[derive(Deserialize, Clone)]
	pub struct FmtPrice {
		// #[serde(rename = "originalPrice")]
		// original: String,
		#[serde(rename = "discountPrice")]
		pub discount: String,
	}

	#[derive(Deserialize, Clone)]
	pub struct CurrencyInfo {
		pub decimals: i8,
	}
	
	// ------------------------------------------------------- PROMOTIONS

	#[derive(Deserialize, Clone)]
	pub struct GamePromotions {
		#[serde(rename = "promotionalOffers")]
		pub promotional_offers: Vec<PromotionalOfferWrapper>
	}

	#[derive(Deserialize, Clone)]
	pub struct PromotionalOfferWrapper {
		#[serde(rename = "promotionalOffers")]
		pub promotional_offers: Vec<PromotionalOffer>
	}

	#[derive(Deserialize, Clone)]
	pub struct PromotionalOffer {
		#[serde(rename = "discountSetting")]
		pub discount_setting: DiscountSetting,
	}

	#[derive(Deserialize, Clone)]
	pub struct DiscountSetting {
		#[serde(rename = "discountType")]
		pub discount_type: String,
		#[serde(rename = "discountPercentage")]
		pub discount_percentage: i8,
	}

}

mod epic_impl {
	use crate::epic::*;

	impl From<GameResult> for GameInfo {
		fn from(value: GameResult) -> Self {
			let url = value.offer_mappings
				.as_ref()
				.and_then(|m| m.first())
				.map(|m| format!("https://store.epicgames.com/en-US/p/{}", m.slug))
				.unwrap_or_default();

			GameInfo {
				name: value.title,
				description: value.description,
				url: url,
				id: value.id,
				price: value.price.total.discount_price,
				price_fmt: value.price.total.fmt_price.discount,
				currency_code: value.price.total.currency_code,	
			}
		}
	}

	impl From<EpicGamesStoreData> for Vec<GameInfo> {
		fn from(value: EpicGamesStoreData) -> Self {
			value.data.catalog.search_store.elements
				.iter()
				.map(|it| it.clone().into())
				.collect()
		}
	}

	impl GameDataset for EpicGamesStoreData {
		fn get_all_games(&self) -> Vec<GameInfo> {
			self.data.catalog.search_store.elements
				.iter()
				.map(|it| GameInfo::from(it.clone()))
				.collect()
		}

		fn get_source<'a>() -> GameSource<'a> {
			GameSource {
				url: "https://store-site-backend-static.ak.epicgames.com/freeGamesPromotions",
				origin: "https://store.epicgames.com/",
				referer: "https://store.epicgames.com/",
			}
		}
	}
}