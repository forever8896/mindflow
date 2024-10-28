<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  interface Note {
    id: number;
    title: string;
    content: string;
  }

  let notes: Note[] = [];
  let activeNoteId: number | null = null;
  let newNoteTitle: string = '';
  let error: string = '';

  async function createNewNote() {
    if (newNoteTitle.trim()) {
      try {
        notes = await invoke('add_note', { title: newNoteTitle, content: '' });
        newNoteTitle = '';
        setActiveNote(notes[notes.length - 1].id);
      } catch (error) {
        console.error('Failed to create note:', error);
      }
    }
  }

  function setActiveNote(id: number) {
    activeNoteId = id;
  }

  async function updateNoteContent(event: Event) {
    const content = (event.target as HTMLTextAreaElement).value;
    const activeNote = notes.find(note => note.id === activeNoteId);
    if (activeNote) {
      try {
        notes = await invoke('update_note', { 
          id: activeNoteId, 
          title: activeNote.title, 
          content 
        });
      } catch (error) {
        console.error('Failed to update note:', error);
      }
    }
  }

  async function deleteNote(id: number) {
    try {
      notes = await invoke('delete_note', { id });
      if (activeNoteId === id) {
        activeNoteId = notes.length > 0 ? notes[0].id : null;
      }
    } catch (error) {
      console.error('Failed to delete note:', error);
    }
  }

  onMount(async () => {
    try {
      notes = await invoke('get_notes');
    } catch (e) {
      error = `Failed to load notes: ${e}`;
      console.error(error);
    }
  });
</script>

<div class="notes-container">
  {#if error}
    <div class="error">{error}</div>
  {/if}
  <div class="notes-sidebar">
    <h2>Notes</h2>
    <div class="new-note">
      <input 
        type="text" 
        bind:value={newNoteTitle} 
        placeholder="New note title"
      />
      <button on:click={createNewNote}>Create</button>
    </div>
    <ul class="notes-list">
      {#each notes as note (note.id)}
        <li 
          class:active={note.id === activeNoteId}
          on:click={() => setActiveNote(note.id)}
        >
          <span class="note-title">{note.title}</span>
          <button class="delete-btn" on:click|stopPropagation={() => deleteNote(note.id)}>×</button>
        </li>
      {/each}
    </ul>
  </div>
  <div class="note-content">
    {#if activeNoteId !== null}
      {#if notes.find(note => note.id === activeNoteId)}
        <h2>{notes.find(note => note.id === activeNoteId)?.title}</h2>
        <textarea 
          value={notes.find(note => note.id === activeNoteId)?.content} 
          on:input={updateNoteContent}
          placeholder="Start typing your note here..."
        ></textarea>
      {:else}
        <p>Note not found</p>
      {/if}
    {:else}
      <p>Select a note or create a new one</p>
    {/if}
  </div>
</div>

<style>
  .notes-container {
    display: flex;
    height: 100%;
    width: 100%;
    overflow: hidden;
  }

  .notes-sidebar {
    width: 250px;
    min-width: 250px;
    display: flex;
    flex-direction: column;
    border-right: 1px solid var(--secondary-color);
    background-color: var(--primary-color);
    overflow: hidden; /* Prevent sidebar content from overflowing */
  }

  h2 {
    margin: 0;
    padding: 1rem;
    font-weight: 600;
  }

  .new-note {
    display: flex;
    padding: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .new-note input {
    flex-grow: 1;
    padding: 0.5rem;
    font-size: 0.9rem;
    border: 1px solid var(--accent-color);
    border-right: none;
    border-radius: 4px 0 0 4px;
    background-color: var(--secondary-color);
    color: black;
    min-width: 0; /* Allow input to shrink below its default minimum width */
  }

  .new-note button {
    padding: 0.5rem;
    font-size: 0.9rem;
    background-color: var(--secondary-color);
    color: var(--primary-color);
    border: 1px solid var(--accent-color);
    border-left: none;
    border-radius: 0 4px 4px 0;
    cursor: pointer;
    transition: background-color 0.3s ease;
    white-space: nowrap; /* Prevent "Create" from wrapping */
  }

  .new-note button:hover {
    background-color: var(--accent-color);
  }

  .notes-list {
    list-style-type: none;
    padding: 0 1rem;
    margin: 0;
    overflow-y: auto;
    flex-grow: 1;
  }

  .notes-list li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    margin-bottom: 0.5rem;
    background-color: var(--primary-color);
    border: 1px solid var(--accent-color);
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .notes-list li:hover {
    background-color: var(--accent-color);
    color: var(--primary-color);
  }

  .notes-list li.active {
    background-color: var(--secondary-color);
    color: var(--primary-color);
  }

  .note-title {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex-grow: 1;
    margin-right: 0.5rem;
  }

  .delete-btn {
    background-color: transparent;
    border: none;
    color: inherit;
    font-size: 1.2rem;
    cursor: pointer;
    padding: 0 0.3rem;
    transition: color 0.3s ease;
  }

  .delete-btn:hover {
    color: #ff4136;
  }

  .note-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    overflow-y: auto;
  }

  .note-content h2 {
    padding: 0;
    margin-bottom: 1rem;
  }

  .note-content textarea {
    flex-grow: 1;
    padding: 0.5rem;
    font-size: 1rem;
    border: 1px solid var(--accent-color);
    border-radius: 4px;
    resize: none;
    background-color: var(--secondary-color);
    color: black;
  }

  .note-content textarea:focus {
    outline: none;
    border-color: var(--secondary-color);
  }

  .note-content p {
    color: var(--secondary-color);
    opacity: 0.7;
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
