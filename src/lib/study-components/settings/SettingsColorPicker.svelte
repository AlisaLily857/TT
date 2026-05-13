<script lang="ts">
  type Props = {
    value: string;
    onChange: (next: string) => void;
    debounceMs?: number;
  };

  let { value = $bindable(), onChange, debounceMs = 500 }: Props = $props();
  let timer: ReturnType<typeof setTimeout> | null = null;

  function onInput(e: Event) {
    const v = (e.target as HTMLInputElement).value;
    value = v;
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      onChange(v);
    }, debounceMs);
  }

  $effect(() => {
    return () => {
      if (timer) {
        clearTimeout(timer);
        onChange(value);
      }
    };
  });
</script>

<label class="color">
  <input type="color" value={value} oninput={onInput} aria-label="Cor" />
  <span class="hex">{value}</span>
</label>

<style>
  .color {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 4px 8px 4px 4px;
    background: color-mix(in oklab, var(--content-bg) 80%, var(--accent) 4%);
    border: 1px solid color-mix(in oklab, var(--content-border) 70%, transparent);
    border-radius: 6px;
  }

  input[type="color"] {
    width: 28px;
    height: 22px;
    border: none;
    padding: 0;
    background: transparent;
    cursor: pointer;
  }

  .hex {
    font-family: ui-monospace, monospace;
    font-size: 11px;
    color: color-mix(in oklab, currentColor 70%, transparent);
    text-transform: uppercase;
  }
</style>
