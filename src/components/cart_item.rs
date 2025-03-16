use thirtyfour::{
    components::{Component, ElementResolver},
    prelude::*,
};

#[derive(Component)]
pub struct CartItem {
    base: WebElement,

    #[by(xpath = ".//div[@class='c_name'][1]/a")]
    name_text: ElementResolver<WebElement>,

    #[by(xpath = ".//div[@class='c_name'][2]/a")]
    author_text: ElementResolver<WebElement>,

    #[by(xpath = ".//div[@class='shop']/div[@class='price']")]
    price_text: ElementResolver<WebElement>,
}

impl CartItem {
    pub async fn get_title(&self) -> anyhow::Result<String> {
        let author = self.author_text.resolve().await?.text().await?;
        let name = self.name_text.resolve().await?.text().await?;
        Ok(format!("{author}.\n{name}"))
    }

    pub async fn get_price(&self) -> anyhow::Result<String> {
        Ok(self.price_text.resolve().await?.text().await?)
    }
}
