use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

use crate::pages::item::ItemPage;

#[derive(Component)]
pub struct GalleryItem {
    base: WebElement,

    #[by(xpath = "./a/div[@class='ssize']")]
    title_text: ElementResolver<WebElement>,

    #[by(xpath = "./div[@class='price']")]
    price_text: ElementResolver<WebElement>,

    #[by(xpath = "./a/div[contains(@id, 'CartButton')]")]
    cart_button: ElementResolver<WebElement>,

    #[by(xpath = "//div[@id='cmodal']/button[@id='close-button']")]
    modal_close_button: ElementResolver<WebElement>,

    #[by(xpath = "./div[@class='heart']")]
    favorite_button: ElementResolver<WebElement>,
}

impl GalleryItem {
    pub async fn get_title(&self) -> anyhow::Result<String> {
        Ok(self.title_text.resolve().await?.text().await?)
    }

    pub async fn get_price(&self) -> anyhow::Result<String> {
        Ok(self.price_text.resolve().await?.text().await?)
    }

    pub async fn goto_page(&self, driver: &WebDriver) -> anyhow::Result<ItemPage> {
        let title = self.title_text.resolve().await?;
        let text = title.text().await?;
        title.scroll_into_view().await?;
        title.click().await?;
        ItemPage::from_current_page(driver, &text).await
    }

    pub async fn add_to_cart(&self) -> anyhow::Result<()> {
        self.cart_button.resolve().await?.click().await?;
        self.modal_close_button.resolve().await?.click().await?;
        Ok(())
    }

    pub async fn toggle_favorite(&self) -> anyhow::Result<()> {
        self.favorite_button.resolve().await?.click().await?;
        Ok(())
    }
}
