use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

use super::{categories::CategoriesPage, gallery::GalleryPage};

#[derive(Component)]
pub struct IndexPage {
    base: WebElement,

    #[by(xpath = "//div[@class='topmenu']/ul/li/a[text()='Каталог']")]
    gallery_link: ElementResolver<WebElement>,

    #[by(xpath = "//span[@class='search-bar']//input[@class='inp scLarge']")]
    search_input: ElementResolver<WebElement>,

    #[by(xpath = "//span[@class='search-bar']//button[@type='submit']")]
    search_button: ElementResolver<WebElement>,
}

impl IndexPage {
    pub async fn open(driver: &WebDriver) -> anyhow::Result<Self> {
        driver.goto("https://artnow.ru/").await?;
        Ok(driver.query(By::Tag("html")).single().await?.into())
    }

    pub async fn goto_categories(self, driver: &WebDriver) -> anyhow::Result<CategoriesPage> {
        self.gallery_link.resolve().await?.click().await?;
        CategoriesPage::from_current_page(driver).await
    }

    pub async fn search(self, driver: &WebDriver, text: &str) -> anyhow::Result<GalleryPage> {
        self.search_input.resolve().await?.send_keys(text).await?;
        self.search_button.resolve().await?.click().await?;
        GalleryPage::from_current_page(driver, "").await
    }
}
