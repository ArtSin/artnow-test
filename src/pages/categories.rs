use anyhow::ensure;
use derive_more::Display;
use thirtyfour::{components::Component, prelude::*};

use super::gallery::GalleryPage;

#[derive(Display)]
pub enum Category {
    #[display("Вышитые картины")]
    EmbroideredPaintings,
    #[display("Батик")]
    Batik,
    #[display("Ювелирное искусство")]
    JewelryArt,
}

impl Category {
    fn page_title(&self) -> String {
        match self {
            Category::JewelryArt => "ювелирные украшения".to_owned(),
            _ => self.to_string().to_lowercase(),
        }
    }
}

#[derive(Component)]
pub struct CategoriesPage {
    base: WebElement,
}

impl CategoriesPage {
    pub async fn from_current_page(driver: &WebDriver) -> anyhow::Result<Self> {
        const TITLE_START: &str = "Каталог работ";

        let page: Self = driver.query(By::Tag("html")).single().await?.into();
        let title = driver.title().await?;
        ensure!(
            title.starts_with(TITLE_START),
            "Categories page title must start with '{TITLE_START}', actual title is '{title}'"
        );
        Ok(page)
    }

    pub async fn goto_category(
        self,
        driver: &WebDriver,
        category: Category,
    ) -> anyhow::Result<GalleryPage> {
        self.base
            .query(By::XPath(format!(
                "//div[@id='main_container']/a/div[text()=' {category}']"
            )))
            .first()
            .await?
            .click()
            .await?;
        GalleryPage::from_current_page(driver, &category.page_title()).await
    }
}
