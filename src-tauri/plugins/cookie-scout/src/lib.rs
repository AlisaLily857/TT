use std::sync::Arc;

use omniget_plugin_sdk::{export_plugin, OmnigetPlugin, PluginHost};
use serde_json::json;

pub struct CookieScoutPlugin {
    host: Option<Arc<dyn PluginHost>>,
}

impl CookieScoutPlugin {
    pub fn new() -> Self {
        Self { host: None }
    }
}

impl Default for CookieScoutPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl OmnigetPlugin for CookieScoutPlugin {
    fn id(&self) -> &str {
        "cookie-scout"
    }

    fn name(&self) -> &str {
        "Cookie Scout"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn initialize(&mut self, host: Arc<dyn PluginHost>) -> anyhow::Result<()> {
        self.host = Some(host);
        Ok(())
    }

    fn handle_command(
        &self,
        command: String,
        _args: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, String>> + Send + 'static>,
    > {
        match command.as_str() {
            "get_cookie_status" => Box::pin(async move {
                Ok(json!({
                    "platforms": [
                        "YouTube",
                        "Bilibili",
                        "Instagram",
                        "Twitter",
                        "TikTok",
                        "Reddit",
                        "Twitch",
                        "Vimeo",
                        "SoundCloud",
                        "Pinterest",
                        "Udemy",
                        "Bluesky"
                    ],
                    "last_export": "2025-05-11T12:00:00Z",
                    "cookie_count": 42
                }))
            }),
            "trigger_cookie_export" => Box::pin(async move { Ok(json!({"success": true})) }),
            _ => Box::pin(async move { Err(format!("Unknown command: {}", command)) }),
        }
    }

    fn commands(&self) -> Vec<String> {
        vec![
            "get_cookie_status".to_string(),
            "trigger_cookie_export".to_string(),
        ]
    }
}

export_plugin!(CookieScoutPlugin::new());
