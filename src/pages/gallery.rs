use anyhow::ensure;
use derive_more::Display;
use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

use crate::components::gallery_item::GalleryItem;

use super::{cart::CartPage, favorites::FavoritesPage};

#[derive(Display)]
pub enum Genre {
    #[display("Городской пейзаж")]
    CityLandscape,
}

#[derive(Component)]
pub struct GalleryPage {
    base: WebElement,

    #[by(xpath = "//div[@class='seform']/ul/li/button[text()='Применить']")]
    filter_apply_button: ElementResolver<WebElement>,

    #[by(xpath = "//div[@id='sa_container']/div[@class='post'][1]")]
    first_item: ElementResolver<WebElement>,

    #[by(xpath = "//div[@class='topheader']/span[@class='fvtico']")]
    favorites_link: ElementResolver<WebElement>,

    #[by(xpath = "//div[@class='topheader']/span[@class='basketico']")]
    cart_link: ElementResolver<WebElement>,
}

impl GalleryPage {
    pub async fn from_current_page(driver: &WebDriver, name: &str) -> anyhow::Result<Self> {
        let page: Self = driver.query(By::Tag("html")).single().await?.into();
        let title = driver.title().await?;
        ensure!(
            title.contains(name),
            "Gallery page title must contain '{name}', actual title is '{title}'"
        );
        Ok(page)
    }

    pub async fn filter_genre(&self, genre: Genre) -> anyhow::Result<()> {
        self.base
            .query(By::XPath(format!(
                "//div[@id='genrebox']/div/label[text()=' {genre}']"
            )))
            .first()
            .await?
            .click()
            .await?;
        self.filter_apply_button.resolve().await?.click().await?;
        Ok(())
    }

    pub async fn get_first_item(&self) -> anyhow::Result<GalleryItem> {
        Ok(self.first_item.resolve().await?.into())
    }

    pub async fn get_item_by_title(&self, author: &str, name: &str) -> anyhow::Result<GalleryItem> {
        Ok(self
            .base
            .query(By::XPath(format!(
                "//div[@id='sa_container']/div[@class='post' and ./a/div[(./text()[1]='{author}.') and (./text()[2]='{name}')]]"
            )))
            .first()
            .await?.into())
    }

    pub async fn goto_favorites(self, driver: &WebDriver) -> anyhow::Result<FavoritesPage> {
        self.favorites_link.resolve().await?.click().await?;
        FavoritesPage::from_current_page(driver).await
    }

    pub async fn goto_cart(self, driver: &WebDriver) -> anyhow::Result<CartPage> {
        self.cart_link.resolve().await?.click().await?;
        CartPage::from_current_page(driver).await
    }
}
