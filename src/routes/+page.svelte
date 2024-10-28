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
      <TodoList />
    {:else if activeSection === 'Notes'}
      <Notes />
    {:else if activeSection === 'Evolution'}
      <Evolution />
    {:else if activeSection === 'Pomodoro'}
      <Pomodoro />
    {:else if activeSection === 'Journal'}
      <Journal bind:this={journalComponent} />
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
  }

  .content {
    flex-grow: 1;
    padding: 2rem;
    overflow-y: auto;
  }
</style>
