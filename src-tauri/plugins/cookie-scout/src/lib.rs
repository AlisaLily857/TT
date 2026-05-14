use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use chrono::Utc;
use omniget_plugin_sdk::{export_plugin, OmnigetPlugin, PluginHost};
use serde::{Deserialize, Serialize};
use serde_json::json;

const SUPPORTED_PLATFORMS: &[(&str, &[&str])] = &[
    ("youtube", &[".youtube.com", ".google.com"]),
    ("bilibili", &[".bilibili.com", ".bilivideo.com"]),
    ("instagram", &[".instagram.com", ".cdninstagram.com"]),
    ("twitter", &[".twitter.com", ".x.com"]),
    ("tiktok", &[".tiktok.com", ".tiktokcdn.com"]),
    ("reddit", &[".reddit.com"]),
    ("twitch", &[".twitch.tv"]),
    ("vimeo", &[".vimeo.com"]),
    ("soundcloud", &[".soundcloud.com"]),
    ("pinterest", &[".pinterest.com"]),
    ("udemy", &[".udemy.com"]),
    ("bluesky", &[".bsky.app"]),
];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ExportRecord {
    platform: String,
    cookie_count: usize,
    timestamp: String,
    source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PluginState {
    export_history: Vec<ExportRecord>,
    total_exports: u64,
    last_export_platform: Option<String>,
}

pub struct CookieScoutPlugin {
    host: Option<Arc<dyn PluginHost>>,
}

impl CookieScoutPlugin {
    pub fn new() -> Self {
        Self { host: None }
    }

    fn data_dir(&self) -> Result<PathBuf, String> {
        let host = self
            .host
            .as_ref()
            .ok_or_else(|| "Plugin not initialized".to_string())?;
        Ok(host.plugin_data_dir("cookie-scout"))
    }

    fn state_path(&self) -> Result<PathBuf, String> {
        Ok(self.data_dir()?.join("state.json"))
    }

    fn load_state(&self) -> Result<PluginState, String> {
        let path = self.state_path()?;
        if !path.exists() {
            return Ok(PluginState::default());
        }
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    fn save_state(&self, state: &PluginState) -> Result<(), String> {
        let path = self.state_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn platform_name(id: &str) -> &'static str {
        match id {
            "youtube" => "YouTube",
            "bilibili" => "Bilibili",
            "instagram" => "Instagram",
            "twitter" => "Twitter/X",
            "tiktok" => "TikTok",
            "reddit" => "Reddit",
            "twitch" => "Twitch",
            "vimeo" => "Vimeo",
            "soundcloud" => "SoundCloud",
            "pinterest" => "Pinterest",
            "udemy" => "Udemy",
            "bluesky" => "Bluesky",
            _ => "Unknown",
        }
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
        args: serde_json::Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<serde_json::Value, String>> + Send + 'static>,
    > {
        let host_opt = self.host.clone();

        match command.as_str() {
            "get_cookie_status" => Box::pin(async move {
                let host = host_opt.ok_or("Plugin not initialized")?;
                let settings = host.get_settings("cookie-scout");
                let platforms: Vec<serde_json::Value> = SUPPORTED_PLATFORMS
                    .iter()
                    .map(|(id, domains)| {
                        json!({
                            "id": id,
                            "name": CookieScoutPlugin::platform_name(id),
                            "domains": domains,
                        })
                    })
                    .collect();
                Ok(json!({
                    "platforms": platforms,
                    "settings": settings,
                    "version": "0.1.0",
                    "plugin_id": "cookie-scout",
                }))
            }),

            "get_export_history" => {
                let state_result = self.load_state();
                Box::pin(async move {
                    let state = state_result?;
                    Ok(json!({
                        "history": state.export_history,
                        "total_exports": state.total_exports,
                        "last_export_platform": state.last_export_platform,
                    }))
                })
            }

            "record_export" => {
                let platform = args
                    .get("platform")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let cookie_count = args
                    .get("cookie_count")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize;
                let source = args
                    .get("source")
                    .and_then(|v| v.as_str())
                    .unwrap_or("plugin")
                    .to_string();

                let mut state = match self.load_state() {
                    Ok(s) => s,
                    Err(e) => return Box::pin(async move { Err(format!("Failed to load state: {}", e)) }),
                };
                let record = ExportRecord {
                    platform: platform.clone(),
                    cookie_count,
                    timestamp: Utc::now().to_rfc3339(),
                    source,
                };
                state.export_history.push(record);
                if state.export_history.len() > 100 {
                    state.export_history.remove(0);
                }
                state.total_exports += 1;
                state.last_export_platform = Some(platform);

                if let Err(e) = self.save_state(&state) {
                    return Box::pin(async move { Err(format!("Failed to save state: {}", e)) });
                }

                let host_opt2 = host_opt.clone();
                Box::pin(async move {
                    if let Some(host) = host_opt2 {
                        let platform_name = CookieScoutPlugin::platform_name(
                            state.last_export_platform.as_deref().unwrap_or(""),
                        );
                        let _ = host.emit_event(
                            "cookie-scout:export",
                            json!({
                                "platform": state.last_export_platform,
                                "total_exports": state.total_exports,
                            }),
                        );
                        let _ = host.show_toast(
                            "success",
                            &format!("Cookies exported for {}", platform_name),
                        );
                    }
                    Ok(json!({ "success": true }))
                })
            }

            "clear_history" => {
                let mut state = match self.load_state() {
                    Ok(s) => s,
                    Err(e) => return Box::pin(async move { Err(format!("Failed to load state: {}", e)) }),
                };
                let cleared_count = state.export_history.len();
                state.export_history.clear();
                state.total_exports = 0;
                state.last_export_platform = None;
                if let Err(e) = self.save_state(&state) {
                    return Box::pin(async move { Err(format!("Failed to save state: {}", e)) });
                }
                Box::pin(async move {
                    Ok(json!({ "success": true, "cleared_count": cleared_count }))
                })
            }

            "get_settings" => Box::pin(async move {
                let host = host_opt.ok_or("Plugin not initialized")?;
                let settings = host.get_settings("cookie-scout");
                Ok(settings)
            }),

            "save_settings" => Box::pin(async move {
                let host = host_opt.ok_or("Plugin not initialized")?;
                host.save_settings("cookie-scout", args)
                    .map_err(|e| e.to_string())?;
                Ok(json!({ "success": true }))
            }),

            "trigger_cookie_export" => {
                let platform = args
                    .get("platform")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let mut state = match self.load_state() {
                    Ok(s) => s,
                    Err(e) => return Box::pin(async move { Err(format!("Failed to load state: {}", e)) }),
                };
                let record = ExportRecord {
                    platform: platform.clone(),
                    cookie_count: 0,
                    timestamp: Utc::now().to_rfc3339(),
                    source: "plugin_manual".to_string(),
                };
                state.export_history.push(record);
                if state.export_history.len() > 100 {
                    state.export_history.remove(0);
                }
                state.total_exports += 1;
                state.last_export_platform = Some(platform.clone());
                if let Err(e) = self.save_state(&state) {
                    return Box::pin(async move { Err(format!("Failed to save state: {}", e)) });
                }

                let host_opt2 = host_opt.clone();
                Box::pin(async move {
                    if let Some(host) = host_opt2 {
                        let _ = host.emit_event(
                            "cookie-scout:export_requested",
                            json!({ "platform": platform }),
                        );
                        let _ = host.show_toast(
                            "info",
                            &format!(
                                "Cookie export requested for {}. Use the browser extension floating button to export cookies.",
                                CookieScoutPlugin::platform_name(&platform)
                            ),
                        );
                    }
                    Ok(json!({
                        "success": true,
                        "message": "Use the browser extension floating button to export cookies",
                    }))
                })
            }

            _ => Box::pin(async move { Err(format!("Unknown command: {}", command)) }),
        }
    }

    fn commands(&self) -> Vec<String> {
        vec![
            "get_cookie_status".to_string(),
            "get_export_history".to_string(),
            "record_export".to_string(),
            "clear_history".to_string(),
            "get_settings".to_string(),
            "save_settings".to_string(),
            "trigger_cookie_export".to_string(),
        ]
    }

    fn shutdown(&self) {
        // Nothing to clean up
    }
}

export_plugin!(CookieScoutPlugin::new());
