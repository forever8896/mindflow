<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  interface Goal {
    id: number;
    title: string;
    motivation: string;
  }

  let goals: Goal[] = [];
  let newGoalTitle: string = '';
  let newGoalMotivation: string = '';
  let error: string = '';

  async function addGoal() {
    if (newGoalTitle.trim() && newGoalMotivation.trim()) {
      try {
        goals = await invoke('add_goal', { title: newGoalTitle, motivation: newGoalMotivation });
        newGoalTitle = '';
        newGoalMotivation = '';
        error = '';
      } catch (e) {
        error = `Failed to add goal: ${e}`;
        console.error(error);
      }
    }
  }

  async function removeGoal(id: number) {
    try {
      goals = await invoke('remove_goal', { id });
      error = '';
    } catch (e) {
      error = `Failed to remove goal: ${e}`;
      console.error(error);
    }
  }

  async function updateGoal(id: number, title: string, motivation: string) {
    try {
      goals = await invoke('update_goal', { id, title, motivation });
      error = '';
    } catch (e) {
      error = `Failed to update goal: ${e}`;
      console.error(error);
    }
  }

  onMount(async () => {
    try {
      goals = await invoke('get_goals');
    } catch (e) {
      error = `Failed to load goals: ${e}`;
      console.error(error);
    }
  });
</script>

<div class="evolution-container">
  <h2>Evolution Goals</h2>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="add-goal-form">
    <input 
      type="text" 
      bind:value={newGoalTitle} 
      placeholder="Enter your long-term goal"
    />
    <textarea 
      bind:value={newGoalMotivation} 
      placeholder="Write a motivational statement for this goal"
    ></textarea>
    <button on:click={addGoal}>Add Goal</button>
  </div>

  <ul class="goals-list">
    {#each goals as goal (goal.id)}
      <li>
        <h3>{goal.title}</h3>
        <p>{goal.motivation}</p>
        <button class="remove-btn" on:click={() => removeGoal(goal.id)}>Remove</button>
      </li>
    {/each}
  </ul>
</div>

<style>
  .evolution-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
  }

  h2 {
    color: var(--secondary-color);
    text-align: center;
  }

  .add-goal-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  input, textarea {
    padding: 0.5rem;
    font-size: 1rem;
    border: 1px solid var(--accent-color);
    border-radius: 4px;
    background-color: var(--secondary-color);
    color: black;
  }

  textarea {
    height: 100px;
    resize: vertical;
  }

  button {
    padding: 0.5rem;
    font-size: 1rem;
    background-color: var(--secondary-color);
    color: var(--primary-color);
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  button:hover {
    background-color: var(--accent-color);
    color: var(--secondary-color);

  }

  .goals-list {
    list-style-type: none;
    padding: 0;
  }

  .goals-list li {
    background-color: var(--secondary-color);
    border: 1px solid var(--accent-color);
    border-radius: 4px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  .goals-list h3 {
    color: black;
    margin-top: 0;
  }

  .goals-list p {
    color: black;
    font-style: italic;
  }

  .remove-btn {
    background-color: #ff4136;
    color: var(--secondary-color);
    border: none;
    padding: 0.3rem 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .error {
    color: #ff4136;
    background-color: #ffeeee;
    border: 1px solid #ff4136;
    border-radius: 4px;
    padding: 0.5rem;
    margin-bottom: 1rem;
  }
</style>
