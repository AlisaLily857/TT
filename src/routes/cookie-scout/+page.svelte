<script lang="ts">
  import { onMount } from "svelte";
  import { invoke, convertFileSrc } from "@tauri-apps/api/core";
  import { pluginInvoke } from "$lib/plugin-invoke";
  import { showToast } from "$lib/stores/toast-store.svelte";

  let iframeRef: HTMLIFrameElement;
  let frontendUrl = $state<string | null>(null);
  let loadError = $state<string | null>(null);
  let isLoading = $state(true);

  onMount(() => {
    loadFrontend();
    const handler = (e: MessageEvent) => handleMessage(e);
    window.addEventListener("message", handler);
    return () => window.removeEventListener("message", handler);
  });

  async function loadFrontend() {
    try {
      const path = await invoke<string>("get_plugin_frontend_path", {
        pluginId: "cookie-scout",
      });
      // Ensure we load index.html from the frontend directory
      const url = convertFileSrc(path);
      frontendUrl = url.endsWith("/") ? url + "index.html" : url + "/index.html";
      isLoading = false;
    } catch (e) {
      loadError = e instanceof Error ? e.message : String(e);
      isLoading = false;
    }
  }

  async function handleMessage(e: MessageEvent) {
    if (!iframeRef || e.source !== iframeRef.contentWindow) return;
    const data = e.data;
    if (!data || typeof data !== "object") return;

    if (data.type === "plugin-invoke") {
      try {
        const result = await pluginInvoke("cookie-scout", data.command, data.args || {});
        iframeRef.contentWindow?.postMessage(
          { type: "plugin-response", id: data.id, result },
          "*"
        );
      } catch (err) {
        iframeRef.contentWindow?.postMessage(
          { type: "plugin-response", id: data.id, error: err instanceof Error ? err.message : String(err) },
          "*"
        );
      }
    } else if (data.type === "system-invoke") {
      try {
        const result = await invoke(data.command, data.args || {});
        iframeRef.contentWindow?.postMessage(
          { type: "system-response", id: data.id, result },
          "*"
        );
      } catch (err) {
        iframeRef.contentWindow?.postMessage(
          { type: "system-response", id: data.id, error: err instanceof Error ? err.message : String(err) },
          "*"
        );
      }
    } else if (data.type === "show-toast") {
      showToast(data.toastType || "info", data.message);
    }
  }
</script>

<div class="plugin-container">
  {#if isLoading}
    <div class="loading">
      <span class="spinner"></span>
      <p>Loading Cookie Scout...</p>
    </div>
  {:else if loadError}
    <div class="error">
      <p>Failed to load plugin frontend:</p>
      <code>{loadError}</code>
      <p class="hint">
        Make sure the Cookie Scout plugin is installed from the Marketplace.
      </p>
    </div>
  {:else if frontendUrl}
    <iframe
      bind:this={iframeRef}
      src={frontendUrl}
      title="Cookie Scout"
      class="plugin-iframe"
      sandbox="allow-scripts"
    />
  {/if}
</div>

<style>
  .plugin-container {
    width: 100%;
    height: 100%;
    min-height: calc(100vh - 60px);
  }

  .plugin-iframe {
    width: 100%;
    height: 100%;
    min-height: calc(100vh - 60px);
    border: none;
    display: block;
  }

  .loading,
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    min-height: calc(100vh - 60px);
    gap: 12px;
    color: #a0a0b0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    padding: 24px;
    text-align: center;
  }

  .error code {
    background: #1a1a2e;
    padding: 8px 16px;
    border-radius: 8px;
    color: #e94560;
    font-size: 0.85rem;
    max-width: 100%;
    word-break: break-word;
  }

  .error .hint {
    color: #7a7a8a;
    font-size: 0.85rem;
    margin-top: 8px;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #2a2a4a;
    border-top-color: #e94560;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
