use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

use crate::components::gallery_item::GalleryItem;

#[derive(Component)]
pub struct FavoritesPage {
    base: WebElement,

    #[by(xpath = "//div[@id='sa_container']/div[@class='post'][1]")]
    first_picture: ElementResolver<WebElement>,
}

impl FavoritesPage {
    pub async fn from_current_page(driver: &WebDriver) -> anyhow::Result<Self> {
        Ok(driver
            .query(By::XPath(
                "html[//div[@id='main_container']/h1/text() = 'Избранное']",
            ))
            .single()
            .await?
            .into())
    }

    pub async fn get_first_picture(&self) -> anyhow::Result<GalleryItem> {
        Ok(self.first_picture.resolve().await?.into())
    }
}
