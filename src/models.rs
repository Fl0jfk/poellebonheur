use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MarketInfo {
    pub date:   Option<String>,
    pub place:  Option<String>,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MenuCategory {
    Starter,
    MainDish,
    Dessert,
}

impl MenuCategory {
    pub fn label(&self) -> &'static str {
        match self {
            MenuCategory::Starter  => "Entrée",
            MenuCategory::MainDish => "Plat principal",
            MenuCategory::Dessert  => "Dessert",
        }
    }
    pub fn emoji(&self) -> &'static str {
        match self {
            MenuCategory::Starter  => "🥗",
            MenuCategory::MainDish => "🍲",
            MenuCategory::Dessert  => "🍮",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MenuItem {
    pub id:          String,
    pub name:        String,
    pub description: String,
    pub photo_url:   Option<String>,
    pub category:    MenuCategory,
    pub price_info:  Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateMenuItemPayload {
    pub name:       String,
    pub description: String,
    pub category:   MenuCategory,
    pub price_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMenuItemPayload {
    pub name:        Option<String>,
    pub description: Option<String>,
    pub category:    Option<MenuCategory>,
    pub price_info:  Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    #[default]
    Pending,
    Viewed,
    Replied,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QuoteRequest {
    pub id:               String,
    pub last_name:        String,
    pub first_name:       String,
    pub phone:            String,
    pub email:            String,
    pub event_date:       String,
    pub event_place:      String,
    pub number_of_people: u32,
    pub starters:         Vec<String>,
    pub main_dish:        String,
    pub desserts:         Vec<String>,
    pub message:          Option<String>,
    pub created_at:       String,
    pub status:           QuoteStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateQuotePayload {
    pub last_name:        String,
    pub first_name:       String,
    pub phone:            String,
    pub email:            String,
    pub event_date:       String,
    pub event_place:      String,
    pub number_of_people: u32,
    pub starters:         Vec<String>,
    pub main_dish:        String,
    pub desserts:         Vec<String>,
    pub message:          Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct MenuData {
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct QuotesData {
    pub quotes: Vec<QuoteRequest>,
}
