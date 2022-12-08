use gloo_console as console;
use js_sys as js;
use wasm_bindgen::prelude::*;

use web_extensions::{self as ext, omnibox::OnInputEnteredDisposition::*};

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn start() {
    utils::set_panic_hook();

    console::info!("Starting background script");

    ext::omnibox::set_default_suggestion(&ext::omnibox::DefaultSuggestResult {
        description: "Type anything to search",
    })
    .unwrap();

    ext::omnibox::on_input_started()
        .add_listener(|| {
            console::debug!("Input started");
        })
        .forget();

    ext::omnibox::on_input_cancelled()
        .add_listener(|| {
            console::debug!("Input cancelled");
        })
        .forget();

    ext::omnibox::on_input_changed()
        .add_listener(|text, suggest| {
            console::debug!("Input changed", text);
        })
        .forget();

    ext::omnibox::on_input_entered()
        .add_listener(|text, disposition| {
            console::debug!("Input entered", text, disposition.to_string());

            let url = format!(
                "https://www.google.com/search?q={}",
                js::encode_uri_component(text).to_string(),
            );

            wasm_bindgen_futures::spawn_local(async move {
                let mut tab_id = None;

                if disposition == CurrentTab {
                    let query = ext::tabs::QueryDetails {
                        active: Some(true),
                        last_focused_window: Some(true),
                        ..Default::default()
                    };

                    match ext::tabs::query(&query).await {
                        Ok(tabs) => {
                            if let [tab, ..] = &tabs[..] {
                                console::debug!(
                                    "current tab",
                                    tab.id.map_or(-1, Into::<i32>::into)
                                );

                                tab_id = tab.id;
                            }
                        }
                        Err(err) => {
                            console::error!("query tabs failed", err.to_string());
                        }
                    }
                }

                if disposition == CurrentTab {
                    console::info!(
                        "open on the current tab",
                        &url,
                        tab_id.map_or(-1, Into::<i32>::into)
                    );

                    match ext::tabs::update(
                        tab_id,
                        ext::tabs::UpdateProperties {
                            url: Some(&url),
                            ..Default::default()
                        },
                    )
                    .await
                    {
                        Ok(tab) => {
                            console::info!(
                                "opened on the current tab",
                                &url,
                                tab.id.map_or(-1, Into::<i32>::into)
                            )
                        }
                        Err(err) => console::error!("update tabs failed", err.to_string()),
                    }
                } else {
                    console::info!("open on a new tab", &url);

                    match ext::tabs::create(ext::tabs::CreateProperties {
                        active: disposition == NewForegroundTab,
                        url: &url,
                    })
                    .await
                    {
                        Ok(tab) => {
                            console::info!(
                                "open on a new tab",
                                tab.id.map_or(-1, Into::<i32>::into)
                            )
                        }
                        Err(err) => {
                            console::error!("create tab failed", err.to_string());
                        }
                    };
                }
            })
        })
        .forget();
}
