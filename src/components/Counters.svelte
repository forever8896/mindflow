<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  interface Counter {
    id: number;
    name: string;
    value: number;
    created_at: string;
    updated_at: string;
  }

  let counters: Counter[] = [];
  let newCounterName: string = '';
  let error: string | null = null;

  async function loadCounters() {
    try {
      const result = await invoke('get_counters');
      counters = (result as Counter[]).sort((a, b) => a.id - b.id);
    } catch (e) {
      error = `Failed to load counters: ${e}`;
    }
  }

  async function addCounter() {
    if (!newCounterName.trim()) {
      error = 'Counter name cannot be empty';
      return;
    }

    try {
      const result = await invoke('add_counter', { name: newCounterName });
      counters = (result as Counter[]).sort((a, b) => a.id - b.id);
      newCounterName = '';
      error = null;
    } catch (e) {
      error = `Failed to add counter: ${e}`;
    }
  }

  async function incrementCounter(id: number) {
    try {
      const result = await invoke('increment_counter', { id });
      counters = (result as Counter[]).sort((a, b) => a.id - b.id);
      error = null;
    } catch (e) {
      error = `Failed to increment counter: ${e}`;
    }
  }

  async function deleteCounter(id: number) {
    if (confirm('Are you sure you want to delete this counter?')) {
      try {
        const result = await invoke('delete_counter', { id });
        counters = (result as Counter[]).sort((a, b) => a.id - b.id);
        error = null;
      } catch (e) {
        error = `Failed to delete counter: ${e}`;
      }
    }
  }

  onMount(loadCounters);
</script>

<div class="counters-container">
  <h1>Counters</h1>

  <div class="add-counter">
    <input
      type="text"
      bind:value={newCounterName}
      placeholder="Enter counter name"
      on:keydown={(e) => e.key === 'Enter' && addCounter()}
    />
    <button on:click={addCounter}>Add Counter</button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="counters-grid">
    {#each counters as counter (counter.id)}
      <div class="counter-card">
        <button class="delete-btn" on:click={() => deleteCounter(counter.id)}>×</button>
        <h3>{counter.name}</h3>
        <div class="counter-value">{counter.value}</div>
        <button class="increment-btn" on:click={() => incrementCounter(counter.id)}>
          Increment
        </button>
      </div>
    {/each}
  </div>
</div>

<style>
  .counters-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
  }

  h1 {
    color: var(--secondary-color);
    margin-bottom: 2rem;
    font-weight: 600;
  }

  .add-counter {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  input {
    flex: 1;
    padding: 0.75rem;
    border: 2px solid var(--secondary-color);
    border-radius: 4px;
    background: transparent;
    color: var(--secondary-color);
    font-family: var(--font-main);
  }

  input::placeholder {
    color: var(--secondary-color);
    opacity: 0.7;
  }

  button {
    padding: 0.75rem 1.5rem;
    background-color: var(--secondary-color);
    border: none;
    border-radius: 4px;
    color: var(--primary-color);
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  button:hover {
    opacity: 0.9;
  }

  .error {
    color: #ff6b6b;
    margin-bottom: 1rem;
    padding: 0.5rem;
    border-radius: 4px;
    background-color: rgba(255, 107, 107, 0.1);
  }

  .counters-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .counter-card {
    position: relative;
    background-color: rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    padding: 1.5rem;
    text-align: center;
    transition: transform 0.2s;
  }

  .counter-card:hover {
    transform: translateY(-2px);
  }

  .counter-card h3 {
    margin: 0 0 1rem 0;
    color: var(--secondary-color);
    font-weight: 400;
  }

  .counter-value {
    font-size: 2.5rem;
    font-weight: 600;
    color: var(--accent-color);
    margin-bottom: 1rem;
  }

  .delete-btn {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    width: 24px;
    height: 24px;
    padding: 0;
    border-radius: 50%;
    background-color: rgba(255, 107, 107, 0.2);
    color: #ff6b6b;
    font-size: 1.2rem;
    line-height: 1;
  }

  .delete-btn:hover {
    background-color: rgba(255, 107, 107, 0.4);
  }

  .increment-btn {
    width: 100%;
    background-color: var(--secondary-color);
    color: var(--primary-color);
  }
</style> 