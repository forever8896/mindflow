<svelte:head>
  <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;600&display=swap" rel="stylesheet">
</svelte:head>

<script lang="ts">
  import Sidebar from '../components/Sidebar.svelte';
  import TodoList from '../components/TodoList.svelte';
  import Notes from '../components/Notes.svelte';
  import Evolution from '../components/Evolution.svelte';
  import Pomodoro from '../components/Pomodoro.svelte';
  import Journal from '../components/Journal.svelte';
  import Counters from '../components/Counters.svelte';
  import { fade } from 'svelte/transition';
  
  let activeSection = 'Todo';
  let journalComponent: Journal;

  async function handleSectionChange(event: CustomEvent<string>) {
    const previousSection = activeSection;
    activeSection = event.detail;
    
    if (activeSection === 'Journal' && journalComponent) {
      await journalComponent.reloadData();
    }
  }
</script>

<div class="app-container">
  <Sidebar {activeSection} on:sectionChange={handleSectionChange} />
  
  <main class="content">
    {#if activeSection === 'Todo'}
      <div transition:fade={{ duration: 200 }}>
        <TodoList />
      </div>
    {:else if activeSection === 'Notes'}
      <div transition:fade={{ duration: 200 }}>
        <Notes />
      </div>
    {:else if activeSection === 'Evolution'}
      <div transition:fade={{ duration: 200 }}>
        <Evolution />
      </div>
    {:else if activeSection === 'Pomodoro'}
      <div transition:fade={{ duration: 200 }}>
        <Pomodoro />
      </div>
    {:else if activeSection === 'Journal'}
      <div transition:fade={{ duration: 200 }}>
        <Journal bind:this={journalComponent} />
      </div>
    {:else if activeSection === 'Counters'}
      <div transition:fade={{ duration: 200 }}>
        <Counters />
      </div>
    {/if}
  </main>
</div>

<style>
  :root {
    --primary-color: #257180;   
    --secondary-color: #F2E5BF; 
    --accent-color: #8AC926;
    
    --font-main: 'Poppins', sans-serif;
  }

  :global(body) {
    background-color: var(--primary-color);
    color: var(--secondary-color);
    font-family: var(--font-main);
    line-height: 1.6;
    font-weight: 300;
    margin: 0;
    padding: 0;
    overflow: hidden;
  }

  /* Custom Scrollbar Styles */
  :global(*::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(*::-webkit-scrollbar-track) {
    background: var(--primary-color);
    border-radius: 4px;
  }

  :global(*::-webkit-scrollbar-thumb) {
    background: var(--secondary-color);
    border-radius: 4px;
    opacity: 0.7;
  }

  :global(*::-webkit-scrollbar-thumb:hover) {
    background: var(--accent-color);
  }

  /* For Firefox */
  :global(*) {
    scrollbar-width: thin;
    scrollbar-color: var(--secondary-color) var(--primary-color);
  }

  .app-container {
    display: flex;
    height: 100vh;
    position: relative;
  }

  .content {
    flex-grow: 1;
    position: relative;
    overflow-y: auto;
  }

  .content > div {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    padding: 2rem;
    box-sizing: border-box;
    min-height: 100%;
  }
</style>
