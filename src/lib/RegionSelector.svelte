<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let backgroundUrl = $state("");
  let selecting = $state(false);
  let startX = $state(0);
  let startY = $state(0);
  let curX = $state(0);
  let curY = $state(0);
  let mouseX = $state(0);
  let mouseY = $state(0);

  // Screen layout info for multi-monitor support
  let monitors = $state([]);
  let originX = $state(0);
  let originY = $state(0);
  let virtualWidth = $state(0);
  let virtualHeight = $state(0);

  // Derived selection rectangle
  let selX = $derived(Math.min(startX, curX));
  let selY = $derived(Math.min(startY, curY));
  let selW = $derived(Math.abs(curX - startX));
  let selH = $derived(Math.abs(curY - startY));

  onMount(async () => {
    try {
      backgroundUrl = await invoke("get_pending_data_url");
      const layout = await invoke("get_screen_layout");
      monitors = layout.monitors;
      originX = layout.origin_x;
      originY = layout.origin_y;
      virtualWidth = layout.virtual_width;
      virtualHeight = layout.virtual_height;
    } catch (e) {
      console.error("Failed to get screenshot:", e);
      await invoke("cancel_region_capture");
    }
  });

  /** Find which monitor index contains the given CSS coordinate */
  function getMonitorAt(cssX, cssY) {
    for (let i = 0; i < monitors.length; i++) {
      const m = monitors[i];
      const mx = m.x - originX;
      const my = m.y - originY;
      if (cssX >= mx && cssX < mx + m.width && cssY >= my && cssY < my + m.height) {
        return i;
      }
    }
    return -1;
  }

  function onKeyDown(e) {
    if (e.key === "Escape") {
      invoke("cancel_region_capture");
    }
  }

  function onMouseDown(e) {
    if (e.button !== 0) return;
    selecting = true;
    startX = e.clientX;
    startY = e.clientY;
    curX = e.clientX;
    curY = e.clientY;
  }

  function onMouseMove(e) {
    mouseX = e.clientX;
    mouseY = e.clientY;
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
      // Composite is in logical pixel space matching CSS pixels — no DPR scaling needed
      await invoke("finish_region_capture", {
        x: Math.round(selX),
        y: Math.round(selY),
        w: Math.round(selW),
        h: Math.round(selH),
      });
    } catch (e) {
      console.error("Region capture failed:", e);
    }
  }

  async function onContextMenu(e) {
    e.preventDefault();
    try {
      if (e.ctrlKey) {
        // Ctrl+Right-click: capture all monitors
        await invoke("capture_full_and_finish");
      } else {
        // Right-click: capture just this monitor
        const idx = getMonitorAt(e.clientX, e.clientY);
        if (idx >= 0) {
          await invoke("finish_monitor_capture", { monitorIndex: idx });
        } else {
          // Cursor in a gap between monitors — fall back to all monitors
          await invoke("capture_full_and_finish");
        }
      }
    } catch (err) {
      console.error("Capture failed:", err);
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="overlay"
  style="background-image: url({backgroundUrl}); width: {virtualWidth}px; height: {virtualHeight}px;"
  onmousedown={onMouseDown}
  onmousemove={onMouseMove}
  onmouseup={onMouseUp}
  oncontextmenu={onContextMenu}
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
      {Math.round(selW)} x {Math.round(selH)}
    </div>
  {/if}

  <!-- Floating tooltip -->
  {#if !selecting}
    <div class="tooltip" style="left:{mouseX + 16}px; top:{mouseY + 16}px;">
      Drag to select region &bull; Right-click for this monitor &bull; Ctrl+Right-click for all monitors &bull; ESC to cancel
    </div>
  {/if}
</div>

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    /* width and height set via inline style from layout */
    background-size: 100% 100%;
    background-position: top left;
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

  .tooltip {
    position: absolute;
    z-index: 20;
    background: rgba(0, 0, 0, 0.8);
    color: #fff;
    font-size: 13px;
    font-family: system-ui, sans-serif;
    padding: 6px 12px;
    border-radius: 4px;
    pointer-events: none;
    white-space: nowrap;
  }
</style>
