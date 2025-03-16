use anyhow::ensure;
use futures::future::join_all;
use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

#[derive(Component)]
pub struct ItemPage {
    base: WebElement,

    #[by(xpath = "//div[@class='imgcontainer']/h1")]
    name_text: ElementResolver<WebElement>,

    #[by(xpath = "//div[@class='imgcontainer']/div[@class='txtline']/a")]
    author_text: ElementResolver<WebElement>,

    #[by(xpath = "//div[@id='main_container']/div/div[@class='txt-p pgray lft']")]
    description_text: ElementResolver<WebElement>,

    #[by(xpath = "//div[@id='main_container']/div/div[text() = 'Ключевые слова: ']/a/span")]
    keywords: ElementResolver<Vec<WebElement>>,

    #[by(xpath = "//div[@class='infocontainer']/div[./span/text()='Стиль: ']/a")]
    style_link: ElementResolver<WebElement>,
}

impl ItemPage {
    pub async fn from_current_page(
        driver: &WebDriver,
        expected_title: &str,
    ) -> anyhow::Result<Self> {
        let page: Self = driver.query(By::Tag("html")).single().await?.into();
        let title = page.get_title().await?;
        ensure!(
            title == expected_title,
            "Picture page title must equal to '{expected_title}', actual title is '{title}'"
        );
        Ok(page)
    }

    pub async fn get_title(&self) -> anyhow::Result<String> {
        let author = self.author_text.resolve().await?.text().await?;
        let name = self.name_text.resolve().await?.text().await?;
        Ok(format!("{author}.\n{name}"))
    }

    pub async fn get_description(&self) -> anyhow::Result<String> {
        Ok(self.description_text.resolve().await?.text().await?)
    }

    pub async fn get_keywords(&self) -> anyhow::Result<Vec<String>> {
        join_all(
            self.keywords
                .resolve()
                .await?
                .into_iter()
                .map(async |x| x.text().await.map_err(|e| e.into())),
        )
        .await
        .into_iter()
        .collect()
    }

    pub async fn get_style(&self) -> anyhow::Result<String> {
        Ok(self.style_link.resolve().await?.text().await?)
    }
}
