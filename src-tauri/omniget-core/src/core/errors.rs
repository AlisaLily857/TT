pub fn classify_download_error(error: &str) -> (&str, &str) {
    let lower = error.to_lowercase();

    if lower.contains("cookie")
        || lower.contains("login")
        || lower.contains("sign in")
        || lower.contains("authentication")
        || lower.contains("403")
    {
        return ("auth_required", "This content requires login. Install the browser extension and visit the site while logged in.");
    }

    if lower.contains("captcha")
        || lower.contains("blocking")
        || lower.contains("rate limit")
        || lower.contains("429")
        || lower.contains("too many")
    {
        return (
            "rate_limited",
            "Too many requests. Try again in a few minutes.",
        );
    }

    if lower.contains("private") || lower.contains("restricted") || lower.contains("age") {
        return ("restricted", "This content is private or age-restricted.");
    }

    if lower.contains("downloaded file") && lower.contains("not found") {
        return (
            "file_missing",
            "Downloaded file could not be located in the output folder.",
        );
    }

    if lower.contains("not found")
        || lower.contains("404")
        || lower.contains("unavailable")
        || lower.contains("deleted")
    {
        return ("not_found", "Content not found or has been deleted.");
    }

    if lower.contains("ffmpeg") || lower.contains("mux") || lower.contains("merge") {
        return (
            "ffmpeg_needed",
            "FFmpeg is required for this download. Install it from Settings.",
        );
    }

    // Only match errors that explicitly say yt-dlp is *missing/unavailable*.
    // Do NOT match runtime errors like "yt-dlp: extractor is broken" or
    // "yt-dlp: Connection timed out" — those mean yt-dlp is installed but
    // failed during execution, so showing "Install it from Settings" is misleading.
    if lower.contains("no downloader")
        || lower.contains("yt-dlp not found")
        || lower.contains("yt-dlp unavailable")
        || lower.contains("yt-dlp not installed")
        || lower.contains("could not find yt-dlp")
    {
        return (
            "ytdlp_needed",
            "yt-dlp is required. Install it from Settings.",
        );
    }

    if lower.contains("nsig") || lower.contains("signature") || lower.contains("cipher") {
        return (
            "ytdlp_outdated",
            "yt-dlp needs updating. Restart the app to auto-update.",
        );
    }

    ("unknown", error)
}
