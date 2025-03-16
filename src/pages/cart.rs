use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

use crate::components::cart_item::CartItem;

#[derive(Component)]
pub struct CartPage {
    base: WebElement,

    #[by(xpath = "//div[@id='main_container']/div[contains(@id, 'cart')][1]")]
    first_item: ElementResolver<WebElement>,
}

impl CartPage {
    pub async fn from_current_page(driver: &WebDriver) -> anyhow::Result<Self> {
        Ok(driver
            .query(By::XPath(
                "html[//div[@id='main_container']/h1/text() = 'Корзина покупок']",
            ))
            .single()
            .await?
            .into())
    }

    pub async fn get_first_item(&self) -> anyhow::Result<CartItem> {
        Ok(self.first_item.resolve().await?.into())
    }
}
