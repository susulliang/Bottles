<script lang="ts">
  import { session, welcomeMessage } from '$lib/store';
  import { api } from '$lib/api';
  import { language, languageNames, translator, type Language } from '$lib/i18n';

  const savedUsernameKey = 'saved_username';
  const savedPassphraseKey = 'saved_passphrase';
  const rememberCredentialsKey = 'remember_credentials';

  let username = '';
  let passphrase = '';
  let error = '';
  let loading = false;
  let rememberCredentials = false;

  if (typeof localStorage !== 'undefined') {
    rememberCredentials = localStorage.getItem(rememberCredentialsKey) === 'true';
    if (rememberCredentials) {
      username = localStorage.getItem(savedUsernameKey) || '';
      passphrase = localStorage.getItem(savedPassphraseKey) || '';
    }
  }

  async function submit() {
    error = '';
    loading = true;
    try {
      const registered = await api.loginOrRegister(username, passphrase);
      if (registered) {
        welcomeMessage.set($translator('welcome'));
      }
      persistCredentials();
      session.set(username);
    } catch (e: any) {
      error = typeof e === 'string' ? e : e?.message || 'error';
    } finally {
      loading = false;
    }
  }

  function persistCredentials() {
    if (typeof localStorage === 'undefined') return;

    localStorage.setItem(rememberCredentialsKey, String(rememberCredentials));
    if (rememberCredentials) {
      localStorage.setItem(savedUsernameKey, username);
      localStorage.setItem(savedPassphraseKey, passphrase);
    } else {
      localStorage.removeItem(savedUsernameKey);
      localStorage.removeItem(savedPassphraseKey);
    }
  }

  function handleRememberToggle() {
    rememberCredentials = !rememberCredentials;
    persistCredentials();
  }
</script>

<div class="auth-container">
  <div class="glass auth-card">
    <div class="language-row">
      {#each Object.entries(languageNames) as [code, label]}
        <button
          type="button"
          class:active-lang={$language === code}
          onclick={() => language.set(code as Language)}
        >{label}</button>
      {/each}
    </div>

    <h1>{$translator('title')}</h1>
    <p class="subtitle">{$translator('subtitle')}</p>

    <form onsubmit={(e) => { e.preventDefault(); submit(); }}>
      <input
        bind:value={username}
        placeholder={$translator('username')}
        maxlength={32}
        pattern="[a-zA-Z0-9_-]+"
        required
      />
      <input
        type="password"
        bind:value={passphrase}
        placeholder={$translator('passphrase')}
        required
      />

      <label class="remember-row">
        <input type="checkbox" checked={rememberCredentials} onchange={handleRememberToggle} />
        <span>{$translator('rememberCredentials')}</span>
      </label>

      {#if error}
        <p class="error">{error}</p>
      {/if}

      <button type="submit" disabled={loading}>
        {loading ? $translator('entering') : $translator('enter')}
      </button>
    </form>
  </div>
</div>

<style>
  .auth-container {
    position: relative;
    z-index: 1;
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .auth-card {
    padding: 2.25rem;
    width: 100%;
    max-width: 420px;
    text-align: center;
  }

  .language-row {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 0.35rem;
    margin-bottom: 1.5rem;
  }

  .language-row button {
    padding: 0.35rem 0.55rem;
    border-radius: 999px;
    background: rgba(255,255,255,0.08);
    border: 1px solid rgba(255,255,255,0.12);
    font-size: 0.75rem;
  }

  .language-row button.active-lang {
    background: rgba(123, 218, 255, 0.28);
    border-color: rgba(255,255,255,0.45);
  }

  h1 {
    margin: 0 0 0.25rem;
    font-size: 2rem;
    color: #fff;
  }

  .subtitle {
    margin: 0 0 2rem;
    color: rgba(255,255,255,0.6);
    font-size: 0.9rem;
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .remember-row {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    color: rgba(255, 255, 255, 0.82);
    font-size: 0.9rem;
    text-align: left;
    user-select: none;
  }

  .remember-row input {
    width: 16px;
    height: 16px;
    margin: 0;
    accent-color: #7bdaff;
  }

  input {
    padding: 0.75rem 1rem;
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 12px;
    background: rgba(255,255,255,0.08);
    color: #fff;
    font-size: 1rem;
    outline: none;
    transition: border-color 0.2s;
  }

  input:focus {
    border-color: rgba(255,255,255,0.5);
  }

  input::placeholder {
    color: rgba(255,255,255,0.4);
  }

  button {
    padding: 0.75rem;
    border: none;
    border-radius: 12px;
    background: rgba(255,255,255,0.15);
    color: #fff;
    font-size: 1rem;
    cursor: pointer;
    transition: background 0.2s;
  }

  button:hover:not(:disabled) {
    background: rgba(255,255,255,0.25);
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error {
    color: #ff6b6b;
    font-size: 0.85rem;
    margin: 0;
  }
</style>
