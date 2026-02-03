<script>
  import RegionSelector from "./lib/RegionSelector.svelte";

  const params = new URLSearchParams(window.location.search);
  const isOverlay = params.get("mode") === "overlay";
</script>

{#if isOverlay}
  <RegionSelector />
{:else}
  <main>
    <p>OpenCap is running in the system tray.</p>
  </main>
{/if}

<style>
  main {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-family: system-ui, sans-serif;
    color: #555;
  }
</style>
