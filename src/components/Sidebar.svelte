<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { fade } from 'svelte/transition';
    import { onMount } from 'svelte';
  
    export let activeSection: string;
  
    const dispatch = createEventDispatcher();
  
    function selectSection(section: string) {
      dispatch('sectionChange', section);
    }
  
    const sections = ['Todo', 'Notes', 'Evolution', 'Pomodoro', 'Counters', 'Journal'];

    let fileOpenMessage = ''; 
    let messageTimeout: number;

    onMount(() => {
      return () => {
        if (messageTimeout) clearTimeout(messageTimeout);
      };
    });

    async function openUserData() {
      try {
        const result = await invoke('open_user_data_file');
        fileOpenMessage = result as string;
        console.log(fileOpenMessage);
      } catch (error) {
        fileOpenMessage = `Error: ${error}`;
        console.error('Failed to open user data file:', error);
      }

      if (messageTimeout) clearTimeout(messageTimeout);
      messageTimeout = setTimeout(() => {
        fileOpenMessage = '';
      }, 5000);
    }
</script>

<aside class="sidebar">
  <nav>
    <ul>
      {#each sections as section}
        <li class:active={activeSection === section}>
          <button on:click={() => selectSection(section)}>{section}</button>
        </li>
      {/each}
    </ul>
  </nav>
  <div class="sidebar-footer">
    <button class="open-data-button" on:click={openUserData}>Open user data</button>
    <div class="message-container">
      {#if fileOpenMessage}
        <p class="file-open-message" transition:fade={{ duration: 300 }}>{fileOpenMessage}</p>
      {/if}
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: 200px;
    background-color: var(--secondary-color);
    height: 100vh;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    font-family: "Poppins", sans-serif;
    font-weight: 500;
  }

  ul {
    list-style-type: none;
    padding: 0;
    flex-grow: 1;
  }

  li {
    margin-bottom: 0.5rem;
    font-weight: 700;
  }

  button {
    width: 100%;
    text-align: left;
    padding: 0.5rem;
    background-color: transparent;
    border: none;
    color: black;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover, .active button {
    background-color: var(--accent-color);
  }

  .sidebar-footer {
    margin-top: auto;
    padding-bottom: 2rem;
  }

  .open-data-button {
    width: 100%;
    text-align: center;
    padding: 0.5rem;
    background-color: white;
    color: black;
    border: none;
    cursor: pointer;
    transition: all 0.3s;
  }

  .open-data-button:hover {
    background-color: var(--accent-color);
    color: white;
  }

  .message-container {
    position: relative;
    height: 4rem;
    margin-top: 0.5rem;
  }

  .file-open-message {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    font-size: 0.8rem;
    word-wrap: break-word;
    color: black;
    background-color: rgba(255, 255, 255, 0.9);
    padding: 0.5rem;
    border-radius: 4px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
</style>
