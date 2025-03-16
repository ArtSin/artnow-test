use thirtyfour::prelude::*;

pub mod components;
pub mod pages;

fn get_selenium_url() -> String {
    std::env::var("SELENIUM_URL").unwrap_or("http://localhost:4444".to_owned())
}

pub struct Context {
    driver: WebDriver,
}

impl Context {
    pub async fn chrome() -> anyhow::Result<Self> {
        let mut caps = DesiredCapabilities::chrome();
        caps.set_headless()?;
        caps.add_arg("--window-size=1280,1024")?;
        let driver = WebDriver::new(get_selenium_url(), caps).await?;
        Ok(Self { driver })
    }

    pub async fn firefox() -> anyhow::Result<Self> {
        let mut caps = DesiredCapabilities::firefox();
        caps.set_headless()?;
        caps.add_arg("--width=1280")?;
        caps.add_arg("--height=1024")?;
        let driver = WebDriver::new(get_selenium_url(), caps).await?;
        Ok(Self { driver })
    }

    async fn screenshot(&self, name: &str) {
        let _ = tokio::fs::create_dir("screenshots").await;
        let path = format!("screenshots/{name}.png");
        if self.driver.screenshot(path.as_ref()).await.is_ok() {
            println!("[[ATTACHMENT|{path}]]");
        }
    }

    pub async fn run<F>(self, f: F, name: &str) -> anyhow::Result<()>
    where
        F: AsyncFn(&WebDriver) -> anyhow::Result<()>,
    {
        let res = f(&self.driver).await;
        self.screenshot(name).await;
        self.driver.quit().await?;
        res
    }
}

#[macro_export]
macro_rules! test_with_browsers {
    ($func:ident, $($browser:ident),*) => {
        $(
            paste::item! {
                #[tokio::test]
                async fn [< $func _ $browser >]() -> anyhow::Result<()> {
                    Context::$browser().await?
                        .run($func, concat!(stringify!($func), "_", stringify!($browser))).await
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! test_with_all_browsers {
    ($func:ident) => {
        test_with_browsers!($func, chrome, firefox);
    };
}
