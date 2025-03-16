use anyhow::ensure;
use artnow_test::{
    Context,
    pages::{categories::Category, gallery::Genre, index::IndexPage},
    test_with_all_browsers, test_with_browsers,
};
use thirtyfour::prelude::*;

/// 1. Перейти в "Вышитые картины", произвести поиск по жанру "Городской пейзаж",
/// проверить, что картина "Трамвайный путь" присутствует в выдаче.
async fn test1(driver: &WebDriver) -> anyhow::Result<()> {
    let index = IndexPage::open(driver).await?;
    let categories = index.goto_categories(driver).await?;
    let embroidered_paintings = categories
        .goto_category(driver, Category::EmbroideredPaintings)
        .await?;
    embroidered_paintings
        .filter_genre(Genre::CityLandscape)
        .await?;
    embroidered_paintings
        .get_item_by_title("Гвоздецкая Татьяна", "Трамвайный путь")
        .await?;
    Ok(())
}
test_with_all_browsers!(test1);

/// 2. Перейти в "Вышитые картины", произвести поиск по жанру "Городской пейзаж",
/// открыть подробности картины "Трамвайный путь", проверить, что стиль картины
/// "Реализм".
async fn test2(driver: &WebDriver) -> anyhow::Result<()> {
    let index = IndexPage::open(driver).await?;
    let categories = index.goto_categories(driver).await?;
    let embroidered_paintings = categories
        .goto_category(driver, Category::EmbroideredPaintings)
        .await?;
    embroidered_paintings
        .filter_genre(Genre::CityLandscape)
        .await?;
    let picture = embroidered_paintings
        .get_item_by_title("Гвоздецкая Татьяна", "Трамвайный путь")
        .await?
        .goto_page(driver)
        .await?;

    const EXPECTED_STYLE: &str = "Реализм";
    let style = picture.get_style().await?;
    ensure!(
        style == EXPECTED_STYLE,
        "Picture style must equal to '{EXPECTED_STYLE}', actual style is '{style}'"
    );
    Ok(())
}
test_with_all_browsers!(test2);

/// 3. Перейти в "Батик", добавить первую картину в избранное, проверить, что
/// выбранная картина сохранилась в разделе "Избранное".
async fn test3(driver: &WebDriver) -> anyhow::Result<()> {
    let index = IndexPage::open(driver).await?;
    let categories = index.goto_categories(driver).await?;
    let batik = categories.goto_category(driver, Category::Batik).await?;

    let picture = batik.get_first_item().await?;
    let expected_title = picture.get_title().await?;
    picture.toggle_favorite().await?;

    let favorites = batik.goto_favorites(driver).await?;
    let favorite_picture = favorites.get_first_picture().await?;
    let title = favorite_picture.get_title().await?;
    ensure!(
        title == expected_title,
        "Favorite picture title must equal to '{expected_title}', actual title is '{title}'"
    );
    Ok(())
}
test_with_all_browsers!(test3);

/// 4. Ввести в поисковую строку "Жираф", проверить, что название первой картины
/// содержит слово "Жираф". (это не будет работать)
/// Проверяется название, описание или ключевые слова первой картины (в любом регистре).
async fn test4(driver: &WebDriver) -> anyhow::Result<()> {
    const SEARCH_TEXT: &str = "Жираф";

    let index = IndexPage::open(driver).await?;
    let search = index.search(driver, SEARCH_TEXT).await?;
    let picture = search.get_first_item().await?.goto_page(driver).await?;

    let search_text_lowercase = SEARCH_TEXT.to_lowercase();
    let title = picture.get_title().await?.to_lowercase();
    let description = picture.get_description().await?.to_lowercase();
    let keywords: Vec<_> = picture
        .get_keywords()
        .await?
        .into_iter()
        .map(|x| x.to_lowercase())
        .collect();
    ensure!(
        title.contains(&search_text_lowercase)
            || description.contains(&search_text_lowercase)
            || keywords.iter().any(|x| x.contains(&search_text_lowercase)),
        "Picture title, description or keywords must contain '{search_text_lowercase}'"
    );
    Ok(())
}
test_with_all_browsers!(test4);

/// 5. Перейти в "Ювелирное искусство", добавить первое изделие в корзину,
/// проверить, что выбранный товар находится в корзине, стоимость товара
/// не изменилась.
async fn test5(driver: &WebDriver) -> anyhow::Result<()> {
    let index = IndexPage::open(driver).await?;
    let categories = index.goto_categories(driver).await?;
    let jewelry_art = categories
        .goto_category(driver, Category::JewelryArt)
        .await?;

    let gallery_item = jewelry_art.get_first_item().await?;
    let expected_title = gallery_item.get_title().await?;
    let expected_price = gallery_item.get_price().await?;
    gallery_item.add_to_cart().await?;

    let cart = jewelry_art.goto_cart(driver).await?;
    let cart_item = cart.get_first_item().await?;
    let title = cart_item.get_title().await?;
    let price = cart_item.get_price().await?;
    ensure!(
        title == expected_title,
        "Cart item title must equal to '{expected_title}', actual title is '{title}'"
    );
    ensure!(
        price == expected_price,
        "Cart item price must equal to '{expected_price}', actual price is '{price}'"
    );
    Ok(())
}
test_with_all_browsers!(test5);
