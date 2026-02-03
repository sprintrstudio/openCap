<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let copyToClipboard = $state(true);
  let autoOpen = $state(true);
  let saveLocally = $state(true);
  let savePath = $state("");
  let openWithProgram = $state("default");
  let programs = $state([]);
  let defaultPath = $state("");
  let saving = $state(false);
  let error = $state("");
  let success = $state(false);

  onMount(async () => {
    try {
      // Load current config
      const config = await invoke("get_config");
      copyToClipboard = config.copyToClipboard;
      autoOpen = config.autoOpen;
      saveLocally = config.saveLocally;
      savePath = config.savePath || "";
      openWithProgram = config.openWithProgram || "default";

      // Get default path
      defaultPath = await invoke("get_default_save_path");
      if (!savePath) {
        savePath = defaultPath;
      }

      // Load available programs
      programs = await invoke("get_image_programs");
    } catch (e) {
      error = `Failed to load settings: ${e}`;
    }
  });

  async function browsePath() {
    try {
      const result = await invoke("browse_folder", { currentPath: savePath || null });
      if (result) {
        savePath = result;
      }
    } catch (e) {
      error = `Failed to browse: ${e}`;
    }
  }

  async function save() {
    // Validation: at least one option must be enabled
    if (!copyToClipboard && !autoOpen && !saveLocally) {
      error = "At least one option must be enabled";
      return;
    }

    saving = true;
    error = "";
    success = false;

    try {
      await invoke("save_config_cmd", {
        config: {
          copyToClipboard,
          autoOpen,
          saveLocally,
          savePath: savePath || null,
          openWithProgram,
        },
      });
      success = true;
      // Close window after short delay
      setTimeout(() => {
        window.close();
      }, 800);
    } catch (e) {
      error = `Failed to save: ${e}`;
    } finally {
      saving = false;
    }
  }

  function resetToDefaults() {
    copyToClipboard = true;
    autoOpen = true;
    saveLocally = true;
    savePath = defaultPath;
    openWithProgram = "default";
    error = "";
    success = false;
  }
</script>

<div class="container">
  <h1>OpenCap Settings</h1>

  <section class="options">
    <h2>After Capture</h2>

    <label class="checkbox-row">
      <input type="checkbox" bind:checked={copyToClipboard} />
      <span>Copy to clipboard</span>
    </label>

    <label class="checkbox-row">
      <input type="checkbox" bind:checked={autoOpen} />
      <span>Open automatically</span>
    </label>

    <label class="checkbox-row">
      <input type="checkbox" bind:checked={saveLocally} />
      <span>Save locally</span>
    </label>
  </section>

  <section class="options">
    <h2>Save Location</h2>
    <div class="path-row">
      <input
        type="text"
        bind:value={savePath}
        placeholder="Screenshots folder path"
        class="path-input"
      />
      <button type="button" onclick={browsePath} class="browse-btn">Browse</button>
    </div>
  </section>

  <section class="options">
    <h2>Open With</h2>
    <select bind:value={openWithProgram} class="program-select">
      {#each programs as program}
        <option value={program.path}>{program.name}</option>
      {/each}
    </select>
  </section>

  {#if error}
    <div class="message error">{error}</div>
  {/if}

  {#if success}
    <div class="message success">Settings saved!</div>
  {/if}

  <div class="buttons">
    <button type="button" onclick={resetToDefaults} class="reset-btn">Reset to Defaults</button>
    <button type="button" onclick={save} disabled={saving} class="save-btn">
      {saving ? "Saving..." : "Save"}
    </button>
  </div>
</div>

<style>
  .container {
    padding: 24px;
    max-width: 420px;
    margin: 0 auto;
  }

  h1 {
    font-size: 20px;
    font-weight: 600;
    margin: 0 0 20px 0;
    color: #fff;
  }

  h2 {
    font-size: 13px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #888;
    margin: 0 0 12px 0;
  }

  section.options {
    margin-bottom: 24px;
  }

  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 0;
    cursor: pointer;
  }

  .checkbox-row input[type="checkbox"] {
    width: 18px;
    height: 18px;
    accent-color: #4a90d9;
    cursor: pointer;
  }

  .checkbox-row span {
    font-size: 14px;
  }

  .path-row {
    display: flex;
    gap: 8px;
  }

  .path-input {
    flex: 1;
    padding: 10px 12px;
    font-size: 13px;
    background: #16213e;
    border: 1px solid #2a3f5f;
    border-radius: 6px;
    color: #eee;
    outline: none;
  }

  .path-input:focus {
    border-color: #4a90d9;
  }

  .browse-btn {
    padding: 10px 16px;
    font-size: 13px;
    background: #2a3f5f;
    border: none;
    border-radius: 6px;
    color: #eee;
    cursor: pointer;
    transition: background 0.2s;
  }

  .browse-btn:hover {
    background: #3a5070;
  }

  .program-select {
    width: 100%;
    padding: 10px 12px;
    font-size: 13px;
    background: #16213e;
    border: 1px solid #2a3f5f;
    border-radius: 6px;
    color: #eee;
    outline: none;
    cursor: pointer;
  }

  .program-select:focus {
    border-color: #4a90d9;
  }

  .message {
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 13px;
    margin-bottom: 16px;
  }

  .message.error {
    background: rgba(220, 53, 69, 0.2);
    border: 1px solid #dc3545;
    color: #ff6b7a;
  }

  .message.success {
    background: rgba(40, 167, 69, 0.2);
    border: 1px solid #28a745;
    color: #5fd97a;
  }

  .buttons {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    margin-top: 24px;
  }

  .reset-btn {
    padding: 12px 20px;
    font-size: 14px;
    background: transparent;
    border: 1px solid #444;
    border-radius: 6px;
    color: #aaa;
    cursor: pointer;
    transition: all 0.2s;
  }

  .reset-btn:hover {
    border-color: #666;
    color: #eee;
  }

  .save-btn {
    padding: 12px 32px;
    font-size: 14px;
    font-weight: 500;
    background: #4a90d9;
    border: none;
    border-radius: 6px;
    color: #fff;
    cursor: pointer;
    transition: background 0.2s;
  }

  .save-btn:hover:not(:disabled) {
    background: #5a9fe9;
  }

  .save-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
