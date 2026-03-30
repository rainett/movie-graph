<script lang="ts">
  import { getConfig, saveConfig, testApiKey, backupProject } from '$lib/services/tauri';
  import { setSoundEnabled } from '$lib/services/sound';
  import type { AppConfig } from '$lib/types/config';

  let { onclose }: { onclose: () => void } = $props();

  let config: AppConfig | null = $state(null);
  let tokenInput = $state('');
  let testStatus = $state<'idle' | 'testing' | 'ok' | 'fail'>('idle');
  let saving = $state(false);
  let loadError = $state(false);
  let backupStatus = $state<'idle' | 'running' | 'ok' | 'fail'>('idle');
  let backupResult = $state<string | null>(null);

  $effect(() => {
    getConfig()
      .then((c) => {
        config = c;
        tokenInput = c.tmdb_read_access_token ?? '';
      })
      .catch(() => {
        loadError = true;
      });
  });

  async function handleTest() {
    const key = tokenInput.trim();
    if (!key || testStatus === 'testing') return;
    testStatus = 'testing';
    try {
      const ok = await testApiKey(key);
      testStatus = ok ? 'ok' : 'fail';
    } catch {
      testStatus = 'fail';
    }
    setTimeout(() => (testStatus = 'idle'), 3000);
  }

  async function handleSave() {
    if (!config || saving) return;
    saving = true;
    try {
      await saveConfig({ ...config, tmdb_read_access_token: tokenInput.trim() || null });
      onclose();
    } finally {
      saving = false;
    }
  }

  function onSoundToggle(e: Event) {
    if (!config) return;
    const enabled = (e.target as HTMLInputElement).checked;
    config = { ...config, sound_enabled: enabled };
    setSoundEnabled(enabled);
  }

  function onAutoSaveChange(e: Event) {
    if (!config) return;
    const val = parseInt((e.target as HTMLSelectElement).value);
    if (val === 0) {
      config = { ...config, auto_save: false };
    } else {
      config = { ...config, auto_save: true, auto_save_interval_ms: val * 1000 };
    }
  }

  async function handleBackup() {
    if (!config || backupStatus === 'running') return;
    // Use recent project path if available
    const projectPath = config.recent_projects[0]?.path?.toString();
    if (!projectPath) {
      backupResult = 'No project to back up';
      backupStatus = 'fail';
      setTimeout(() => { backupStatus = 'idle'; backupResult = null; }, 3000);
      return;
    }
    backupStatus = 'running';
    try {
      const path = await backupProject(projectPath);
      backupResult = path;
      backupStatus = 'ok';
    } catch (e: unknown) {
      backupResult = typeof e === 'string' ? e : 'Backup failed';
      backupStatus = 'fail';
    }
    setTimeout(() => { backupStatus = 'idle'; backupResult = null; }, 5000);
  }

  const autoSaveValue = $derived.by(() => {
    if (!config) return '0';
    if (!config.auto_save) return '0';
    return String(Math.round(config.auto_save_interval_ms / 1000));
  });

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose();
  }

  const testLabel = $derived(
    testStatus === 'testing' ? 'TESTING...' :
    testStatus === 'ok' ? '✓ VALID' :
    testStatus === 'fail' ? '✗ INVALID' : 'TEST KEY'
  );
</script>

<svelte:window onkeydown={onKeyDown} />

<div class="modal-overlay" role="dialog" aria-modal="true" aria-label="Settings">
  <div class="modal">
    <div class="modal-header">
      <span class="modal-title">SETTINGS</span>
      <div class="modal-led" class:led-green={!loadError}></div>
    </div>

    <div class="modal-body">
    {#if loadError}
      <div class="error-msg">Failed to load configuration.</div>
    {:else if !config}
      <div class="loading-msg">LOADING...</div>
    {:else}
      <div class="modal-section">
        <div class="section-label">TMDB API</div>

        <div class="modal-field">
          <label class="modal-label" for="tmdb-token">READ ACCESS TOKEN</label>
          <input
            id="tmdb-token"
            class="modal-input"
            type="password"
            placeholder="eyJ... (leave blank to use default)"
            bind:value={tokenInput}
            onkeydown={(e) => e.key === 'Enter' && handleSave()}
          />
          <div class="field-hint">
            Get a free token at themoviedb.org → Settings → API
          </div>
        </div>

        <button
          class="test-btn"
          class:test-ok={testStatus === 'ok'}
          class:test-fail={testStatus === 'fail'}
          onclick={handleTest}
          disabled={!tokenInput.trim() || testStatus === 'testing'}
        >{testLabel}</button>
      </div>

      <div class="modal-section">
        <div class="section-label">PREFERENCES</div>

        <div class="pref-row">
          <span class="pref-label">SOUND</span>
          <label class="toggle-wrap">
            <input
              class="toggle-input"
              type="checkbox"
              checked={config.sound_enabled}
              onchange={onSoundToggle}
            />
            <span class="toggle-track">
              <span class="toggle-thumb"></span>
            </span>
            <span class="toggle-text">{config.sound_enabled ? 'ON' : 'OFF'}</span>
          </label>
        </div>

        <div class="pref-row">
          <span class="pref-label">AUTO-SAVE</span>
          <select
            class="pref-select"
            value={autoSaveValue}
            onchange={onAutoSaveChange}
          >
            <option value="0">OFF</option>
            <option value="30">30 SEC</option>
            <option value="60">1 MIN</option>
            <option value="300">5 MIN</option>
          </select>
        </div>
      </div>

      <div class="modal-section">
        <div class="section-label">PROJECT</div>
        <button
          class="backup-btn"
          class:backup-ok={backupStatus === 'ok'}
          class:backup-fail={backupStatus === 'fail'}
          onclick={handleBackup}
          disabled={backupStatus === 'running'}
        >
          {backupStatus === 'running' ? 'BACKING UP...' :
           backupStatus === 'ok' ? '✓ BACKUP CREATED' :
           backupStatus === 'fail' ? '✗ BACKUP FAILED' : 'BACKUP PROJECT'}
        </button>
        {#if backupResult && backupStatus !== 'running'}
          <div class="backup-path">{backupResult}</div>
        {/if}
      </div>
    {/if}
    </div>

    <div class="modal-actions">
      <button class="modal-btn-cancel" onclick={onclose}>CANCEL</button>
      <button
        class="modal-btn-ok"
        onclick={handleSave}
        disabled={saving || !config}
      >{saving ? 'SAVING...' : 'SAVE'}</button>
    </div>
  </div>
</div>

<style>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal {
  background: var(--color-device-body);
  border: 2px solid var(--color-device-bezel);
  border-radius: 6px;
  width: 360px;
  max-height: 85vh;
  display: flex;
  flex-direction: column;
  gap: 0;
  overflow: hidden;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #2a2a38 transparent;
}

.modal-header {
  display: flex;
  align-items: center;
  height: 36px;
  padding: 0 14px;
  background: linear-gradient(180deg, #3a3a40 0%, #2d2d32 100%);
  border-bottom: 1px solid #1a1a20;
  flex-shrink: 0;
}

.modal-title {
  color: var(--color-text-secondary);
  font-family: var(--font-ui);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0.2em;
}

.modal-led {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-led-off);
  margin-left: auto;
}

.modal-led.led-green {
  background: var(--color-led-green);
  box-shadow: 0 0 6px var(--color-led-green);
}

.modal-section {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  border-bottom: 1px solid #1a1a20;
}

.section-label {
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.2em;
  border-bottom: 1px solid #2a2a38;
  padding-bottom: 6px;
}

.modal-field {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.modal-label {
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.12em;
}

.modal-input {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 12px;
  outline: none;
  padding: 7px 10px;
  caret-color: var(--color-accent-primary);
  transition: border-color var(--duration-fast);
  width: 100%;
  box-sizing: border-box;
}

.modal-input:focus { border-color: var(--color-accent-primary); }

.field-hint {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 9px;
  opacity: 0.7;
}

.test-btn {
  background: transparent;
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.12em;
  padding: 5px 12px;
  align-self: flex-start;
  transition: color var(--duration-fast), border-color var(--duration-fast);
}

.test-btn:hover:not(:disabled) {
  color: var(--color-text-secondary);
  border-color: #3a3a50;
}

.test-btn:disabled { opacity: 0.4; cursor: default; }

.test-btn.test-ok {
  color: var(--color-led-green);
  border-color: rgba(48, 255, 80, 0.4);
}

.test-btn.test-fail {
  color: #ff6060;
  border-color: rgba(255, 80, 80, 0.4);
}

.modal-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 12px 16px;
}

.modal-btn-cancel {
  background: transparent;
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 10px;
  letter-spacing: 0.1em;
  padding: 6px 14px;
  transition: color var(--duration-fast);
}

.modal-btn-cancel:hover { color: var(--color-text-secondary); }

.modal-btn-ok {
  background: var(--color-accent-primary);
  border: none;
  border-radius: 2px;
  color: #fff;
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.1em;
  padding: 6px 14px;
  transition: opacity var(--duration-fast);
}

.modal-btn-ok:hover:not(:disabled) { opacity: 0.85; }
.modal-btn-ok:disabled { opacity: 0.5; cursor: default; }

.loading-msg, .error-msg {
  padding: 20px 16px;
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 11px;
  letter-spacing: 0.1em;
}

.error-msg { color: #ff6060; }

/* Preferences */
.pref-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
}

.pref-label {
  color: var(--color-text-disabled);
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.12em;
}

.pref-select {
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-screen);
  font-family: var(--font-mono);
  font-size: 11px;
  outline: none;
  padding: 4px 8px;
  cursor: pointer;
}

.toggle-wrap {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.toggle-input {
  display: none;
}

.toggle-track {
  width: 32px;
  height: 16px;
  background: #2a2a38;
  border-radius: 8px;
  position: relative;
  transition: background var(--duration-normal);
}

.toggle-input:checked + .toggle-track {
  background: var(--color-accent-primary);
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #fff;
  transition: transform var(--duration-normal);
}

.toggle-input:checked + .toggle-track .toggle-thumb {
  transform: translateX(16px);
}

.toggle-text {
  color: var(--color-text-secondary);
  font-family: var(--font-mono);
  font-size: 10px;
  letter-spacing: 0.1em;
  width: 24px;
}

/* Backup */
.backup-btn {
  background: transparent;
  border: 1px solid #2a2a38;
  border-radius: 2px;
  color: var(--color-text-disabled);
  cursor: pointer;
  font-family: var(--font-ui);
  font-size: 9px;
  letter-spacing: 0.12em;
  padding: 5px 12px;
  align-self: flex-start;
  transition: color var(--duration-fast), border-color var(--duration-fast);
}

.backup-btn:hover:not(:disabled) {
  color: var(--color-text-secondary);
  border-color: #3a3a50;
}

.backup-btn:disabled { opacity: 0.5; cursor: default; }

.backup-btn.backup-ok {
  color: var(--color-led-green);
  border-color: rgba(48, 255, 80, 0.4);
}

.backup-btn.backup-fail {
  color: #ff6060;
  border-color: rgba(255, 80, 80, 0.4);
}

.backup-path {
  color: var(--color-text-disabled);
  font-family: var(--font-mono);
  font-size: 9px;
  opacity: 0.7;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-top: 4px;
}
</style>
