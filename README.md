# Зависимости

- Rust 1.85
- cargo-nextest
- Allure из https://github.com/allure-framework/allure2/pull/2697 (https://drive.google.com/file/d/1lOlArVDKW6Vw6wv2iY6bo3ZT-0W_0ivp/view?usp=sharing)
- Chrome + chromedriver
- Firefox + geckodriver
- Selenium

# Запуск
## Docker

1. Распаковать allure в каталог проекта
2. `docker compose run --build artnow-test`
3. Открыть `allure-report/index.html`

## Вне docker

1. Установить зависимости
2. `java -jar .../selenium-server-4.29.0.jar standalone --host localhost --port 4444`
3. `rm -rf screenshots allure-report`
4. `cargo nextest run --release --profile junit --no-fail-fast -j 4`
5. `.../allure-2.31-SNAPSHOT/bin/allure generate target/nextest/junit/`
6. `.../allure-2.31-SNAPSHOT/bin/allure open allure-report`