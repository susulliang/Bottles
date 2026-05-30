<script lang="ts">
  import { session } from '$lib/store';
  import { api } from '$lib/api';

  let username = '';
  let passphrase = '';
  let isLogin = true;
  let error = '';
  let loading = false;

  async function submit() {
    error = '';
    loading = true;
    try {
      if (isLogin) {
        await api.login(username, passphrase);
      } else {
        await api.register(username, passphrase);
      }
      session.set(username);
    } catch (e: any) {
      error = typeof e === 'string' ? e : e?.message || 'error';
    } finally {
      loading = false;
    }
  }
</script>

<div class="auth-container">
  <div class="glass auth-card">
    <h1>漂流瓶</h1>
    <p class="subtitle">Bottles</p>

    <form onsubmit={(e) => { e.preventDefault(); submit(); }}>
      <input
        bind:value={username}
        placeholder="Username"
        maxlength={32}
        pattern="[a-zA-Z0-9_-]+"
        required
      />
      <input
        type="password"
        bind:value={passphrase}
        placeholder="Passphrase"
        required
      />

      {#if error}
        <p class="error">{error}</p>
      {/if}

      <button type="submit" disabled={loading}>
        {loading ? '...' : isLogin ? 'Login' : 'Register'}
      </button>
    </form>

    <button class="link" onclick={() => { isLogin = !isLogin; error = ''; }}>
      {isLogin ? 'No account? Register' : 'Already have an account? Login'}
    </button>
  </div>
</div>

<style>
  .auth-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .auth-card {
    padding: 2.5rem;
    width: 100%;
    max-width: 400px;
    text-align: center;
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

  .link {
    background: none;
    color: rgba(255,255,255,0.6);
    font-size: 0.85rem;
    margin-top: 0.5rem;
  }

  .link:hover {
    color: #fff;
  }

  .error {
    color: #ff6b6b;
    font-size: 0.85rem;
    margin: 0;
  }
</style>
