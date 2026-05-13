<script lang="ts">
  type Props = {
    value: number;
    min: number;
    max: number;
    step?: number;
    onChange: (next: number) => void;
    debounceMs?: number;
  };

  let { value = $bindable(), min, max, step = 1, onChange, debounceMs = 500 }: Props = $props();
  let timer: ReturnType<typeof setTimeout> | null = null;

  function flush(next: number) {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      onChange(next);
    }, debounceMs);
  }

  function onInput(e: Event) {
    const v = Number((e.target as HTMLInputElement).value);
    value = v;
    flush(v);
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

<input
  type="range"
  {min}
  {max}
  {step}
  value={value}
  oninput={onInput}
  class="slider"
  aria-valuemin={min}
  aria-valuemax={max}
  aria-valuenow={value}
/>

<style>
  .slider {
    width: 160px;
    height: 18px;
    accent-color: var(--accent);
  }
</style>
