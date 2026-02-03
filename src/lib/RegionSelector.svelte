<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let backgroundUrl = $state("");
  let selecting = $state(false);
  let startX = $state(0);
  let startY = $state(0);
  let curX = $state(0);
  let curY = $state(0);

  // Derived selection rectangle
  let selX = $derived(Math.min(startX, curX));
  let selY = $derived(Math.min(startY, curY));
  let selW = $derived(Math.abs(curX - startX));
  let selH = $derived(Math.abs(curY - startY));

  onMount(async () => {
    try {
      backgroundUrl = await invoke("get_pending_data_url");
    } catch (e) {
      console.error("Failed to get screenshot:", e);
      await invoke("cancel_region_capture");
    }
  });

  function onKeyDown(e) {
    if (e.key === "Escape") {
      invoke("cancel_region_capture");
    }
  }

  function onMouseDown(e) {
    selecting = true;
    startX = e.clientX;
    startY = e.clientY;
    curX = e.clientX;
    curY = e.clientY;
  }

  function onMouseMove(e) {
    if (selecting) {
      curX = e.clientX;
      curY = e.clientY;
    }
  }

  async function onMouseUp() {
    if (!selecting) return;
    selecting = false;

    if (selW < 5 || selH < 5) return; // ignore tiny selections

    try {
      // Account for device pixel ratio for HiDPI
      const dpr = window.devicePixelRatio || 1;
      await invoke("finish_region_capture", {
        x: Math.round(selX * dpr),
        y: Math.round(selY * dpr),
        w: Math.round(selW * dpr),
        h: Math.round(selH * dpr),
      });
    } catch (e) {
      console.error("Region capture failed:", e);
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="overlay"
  style="background-image: url({backgroundUrl})"
  onmousedown={onMouseDown}
  onmousemove={onMouseMove}
  onmouseup={onMouseUp}
>
  <!-- Dim layer -->
  <div class="dim"></div>

  <!-- Clear selection rectangle -->
  {#if selecting && selW > 0 && selH > 0}
    <div
      class="selection"
      style="left:{selX}px; top:{selY}px; width:{selW}px; height:{selH}px;"
    ></div>
  {/if}

  <!-- Size indicator -->
  {#if selecting && selW > 0 && selH > 0}
    <div class="size-label" style="left:{selX}px; top:{selY + selH + 4}px;">
      {Math.round(selW * (window.devicePixelRatio || 1))} x {Math.round(selH * (window.devicePixelRatio || 1))}
    </div>
  {/if}
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background-size: cover;
    background-position: center;
    cursor: crosshair;
    user-select: none;
  }

  .dim {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.35);
    pointer-events: none;
  }

  .selection {
    position: absolute;
    border: 2px solid #fff;
    background: transparent;
    box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.35);
    z-index: 10;
    pointer-events: none;
  }

  .size-label {
    position: absolute;
    z-index: 11;
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    font-size: 12px;
    font-family: system-ui, sans-serif;
    padding: 2px 6px;
    border-radius: 3px;
    pointer-events: none;
  }
</style>
