/// SSRF and path-traversal validation utilities.

/// Rejects URLs that resolve to loopback, link-local, or private IP addresses,
/// preventing Server-Side Request Forgery (SSRF) attacks.
pub fn is_url_safe_for_download(url: &str) -> bool {
    let parsed = match url::Url::parse(url) {
        Ok(u) => u,
        Err(_) => return false,
    };

    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return false;
    }

    if let Some(host) = parsed.host_str() {
        let lower = host.to_lowercase();

        // Reject localhost variants
        if lower == "localhost"
            || lower == "127.0.0.1"
            || lower == "::1"
            || lower.starts_with("127.")
        {
            return false;
        }

        // Reject IPv6 loopback
        if lower == "0:0:0:0:0:0:0:1" || lower == "[::1]" {
            return false;
        }

        // Reject link-local (169.254.x.x)
        if lower.starts_with("169.254.") {
            return false;
        }

        // Reject IPv6 link-local (fe80::/10)
        if lower.starts_with("fe80:") || lower.starts_with("[fe80:") {
            return false;
        }

        // Reject private IPv4 ranges
        if is_private_ipv4(host) {
            return false;
        }

        // Reject IPv6 unique local addresses (fc00::/7)
        if lower.starts_with("fc") || lower.starts_with("fd") {
            if lower.contains(':') {
                return false;
            }
        }
    }

    true
}

fn is_private_ipv4(host: &str) -> bool {
    // 10.0.0.0/8
    if host.starts_with("10.") {
        return true;
    }

    // 172.16.0.0/12
    if host.starts_with("172.") {
        if let Some(dot) = host.find('.') {
            if let Some(second_dot) = host[dot + 1..].find('.') {
                let second_octet: u8 = host[dot + 1..dot + 1 + second_dot].parse().unwrap_or(0);
                if (16..=31).contains(&second_octet) {
                    return true;
                }
            }
        }
    }

    // 192.168.0.0/16
    if host.starts_with("192.168.") {
        return true;
    }

    // 100.64.0.0/10 (CGNAT)
    if host.starts_with("100.") {
        if let Some(dot) = host.find('.') {
            if let Some(second_dot) = host[dot + 1..].find('.') {
                let second_octet: u8 = host[dot + 1..dot + 1 + second_dot].parse().unwrap_or(0);
                if (64..=127).contains(&second_octet) {
                    return true;
                }
            }
        }
    }

    // 127.0.0.0/8 (already handled by prefix check, but be thorough)
    if host.starts_with("127.") {
        return true;
    }

    false
}

/// Validates that a path does not contain path-traversal sequences.
/// Returns true if the path is safe (no `..` components, no absolute paths).
pub fn is_path_safe(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    // Reject absolute paths
    if path.starts_with('/') {
        return false;
    }

    #[cfg(target_os = "windows")]
    {
        if path.len() >= 2 && path.as_bytes()[1] == b':' {
            return false;
        }
        if path.starts_with("\\\\") {
            return false;
        }
    }

    // Reject path traversal components
    for component in path.split(|c| c == '/' || c == '\\') {
        if component == ".." {
            return false;
        }
    }

    // Reject null bytes
    if path.contains('\0') {
        return false;
    }

    true
}

/// Validates that a filename template does not contain path separators or traversal.
pub fn is_filename_template_safe(template: &str) -> bool {
    if template.is_empty() {
        return true;
    }

    // Reject absolute paths
    if template.starts_with('/') {
        return false;
    }

    #[cfg(target_os = "windows")]
    {
        if template.len() >= 2 && template.as_bytes()[1] == b':' {
            return false;
        }
        if template.starts_with("\\\\") {
            return false;
        }
    }

    // Reject path traversal
    for component in template.split(|c| c == '/' || c == '\\') {
        if component == ".." {
            return false;
        }
    }

    // Reject drive-relative on Windows (e.g., C:file)
    #[cfg(target_os = "windows")]
    {
        if template.len() >= 2
            && template.as_bytes()[1] == b':'
            && template.as_bytes()[0].is_ascii_alphabetic()
        {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssrf_localhost() {
        assert!(!is_url_safe_for_download("http://localhost/secret"));
        assert!(!is_url_safe_for_download("http://127.0.0.1/secret"));
        assert!(!is_url_safe_for_download("http://::1/secret"));
    }

    #[test]
    fn test_ssrf_private_ip() {
        assert!(!is_url_safe_for_download("http://10.0.0.1/secret"));
        assert!(!is_url_safe_for_download("http://192.168.1.1/secret"));
        assert!(!is_url_safe_for_download("http://172.16.0.1/secret"));
        assert!(!is_url_safe_for_download("http://172.31.255.255/secret"));
        assert!(!is_url_safe_for_download("http://169.254.1.1/secret"));
    }

    #[test]
    fn test_ssrf_public_ip() {
        assert!(is_url_safe_for_download("http://8.8.8.8/"));
        assert!(is_url_safe_for_download("https://www.youtube.com/watch?v=abc"));
    }

    #[test]
    fn test_path_traversal() {
        assert!(!is_path_safe("../../../etc/passwd"));
        assert!(!is_path_safe("/etc/passwd"));
        assert!(is_path_safe("downloads/videos"));
    }
}
