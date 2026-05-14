(function () {
  "use strict";

  const PLATFORM_NAMES = {
    youtube: "YouTube",
    bilibili: "Bilibili",
    instagram: "Instagram",
    twitter: "Twitter",
    tiktok: "TikTok",
    reddit: "Reddit",
    twitch: "Twitch",
    vimeo: "Vimeo",
    soundcloud: "SoundCloud",
    pinterest: "Pinterest",
    udemy: "Udemy",
    bluesky: "Bluesky",
  };

  function detectPlatform() {
    const host = location.hostname.toLowerCase();
    if (host === "youtu.be" || host.endsWith(".youtube.com")) return "youtube";
    if (host === "b23.tv" || host.endsWith(".bilibili.com")) return "bilibili";
    if (host.endsWith(".instagram.com")) return "instagram";
    if (host.endsWith(".twitter.com") || host.endsWith(".x.com")) return "twitter";
    if (host.endsWith(".tiktok.com")) return "tiktok";
    if (host.endsWith(".reddit.com")) return "reddit";
    if (host.endsWith(".twitch.tv")) return "twitch";
    if (host.endsWith(".vimeo.com")) return "vimeo";
    if (host.endsWith(".soundcloud.com")) return "soundcloud";
    if (host === "pin.it" || host.endsWith(".pinterest.com")) return "pinterest";
    if (host.endsWith(".udemy.com")) return "udemy";
    if (host.endsWith(".bsky.app") || host.endsWith(".bsky.social")) return "bluesky";
    return null;
  }

  function escapeHtml(str) {
    const d = document.createElement("div");
    d.textContent = str;
    return d.innerHTML;
  }

  function createButton(platform) {
    const name = PLATFORM_NAMES[platform] || platform;
    const btn = document.createElement("button");
    btn.className = "omnibox-cookie-scout";
    btn.setAttribute("aria-label", `Export ${name} cookies`);
    btn.innerHTML = `<span class="omnibox-cookie-scout-icon">\u{1F4CB}</span><span class="omnibox-cookie-scout-label">${escapeHtml(name)} Cookie</span>`;
    return btn;
  }

  function init() {
    if (typeof chrome === "undefined" || !chrome.runtime?.sendMessage) return;

    const platform = detectPlatform();
    if (!platform) return;

    if (document.getElementById("omnibox-cookie-scout-root")) return;

    const root = document.createElement("div");
    root.id = "omnibox-cookie-scout-root";
    root.className = "omnibox-cookie-scout-root";

    const btn = createButton(platform);
    root.appendChild(btn);

    const body = document.body;
    if (!body) return;
    body.appendChild(root);

    let isDragging = false;
    let hasMoved = false;
    let startX, startY, startLeft, startTop;

    btn.addEventListener("mousedown", (e) => {
      isDragging = true;
      hasMoved = false;
      startX = e.clientX;
      startY = e.clientY;
      const rect = root.getBoundingClientRect();
      startLeft = rect.left;
      startTop = rect.top;

      function onMouseMove(ev) {
        const dx = ev.clientX - startX;
        const dy = ev.clientY - startY;
        if (Math.abs(dx) > 3 || Math.abs(dy) > 3) {
          hasMoved = true;
        }
        root.style.left = `${startLeft + dx}px`;
        root.style.top = `${startTop + dy}px`;
        root.style.right = "auto";
        root.style.bottom = "auto";
      }

      function onMouseUp() {
        isDragging = false;
        document.removeEventListener("mousemove", onMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
      }

      document.addEventListener("mousemove", onMouseMove);
      document.addEventListener("mouseup", onMouseUp);
    });

    btn.addEventListener("click", () => {
      if (hasMoved) return;

      const label = btn.querySelector(".omnibox-cookie-scout-label");
      btn.classList.add("omnibox-cookie-scout-sending");
      if (label) label.textContent = "Exporting...";

      chrome.runtime.sendMessage({ type: "cookies:manual-export", platform }, (response) => {
        btn.classList.remove("omnibox-cookie-scout-sending");
        if (response?.ok) {
          btn.classList.add("omnibox-cookie-scout-success");
          if (label) label.textContent = "Exported!";
          setTimeout(() => {
            btn.classList.remove("omnibox-cookie-scout-success");
            if (label) label.textContent = `${escapeHtml(PLATFORM_NAMES[platform] || platform)} Cookie`;
          }, 2000);
        } else {
          btn.classList.add("omnibox-cookie-scout-error");
          if (label) label.textContent = "Failed";
          setTimeout(() => {
            btn.classList.remove("omnibox-cookie-scout-error");
            if (label) label.textContent = `${escapeHtml(PLATFORM_NAMES[platform] || platform)} Cookie`;
          }, 2000);
        }
      });
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
