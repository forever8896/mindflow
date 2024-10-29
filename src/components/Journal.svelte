<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { format, parseISO, startOfDay, isToday } from 'date-fns';

  let currentDate = startOfDay(new Date());
  let formattedDate = format(currentDate, 'yyyy-MM-dd');
  let journalContent = '';
  let todos: any[] = [];
  let completedTodos: any[] = [];
  let goals: any[] = [];
  let notes: any[] = [];
  let existingEntry: any = null;
  let allJournalEntries: any[] = [];
  let selectedEntryDate = formattedDate;
  let pomodoroSessions: any[] = [];  // Add this line
  let counters: any[] = [];  // Add counters to the data load

  // Add event bus for real-time updates
  let unsubscribeInterval: number;

  onMount(async () => {
    await loadData();
    unsubscribeInterval = setInterval(async () => {
      if (isToday(currentDate)) {  // Only auto-update if viewing today's entry
        await loadData();
      }
    }, 5000);
  });

  onDestroy(() => {
    if (unsubscribeInterval) {
      clearInterval(unsubscribeInterval);
    }
  });

  async function loadData() {
    // Load data first
    const [newTodos, newGoals, newNotes, newJournalEntries, newPomodoroSessions, newCounters]: any[] = await Promise.all([
      invoke('get_todos'),
      invoke('get_goals'),
      invoke('get_notes'),
      invoke('get_journal_entries'),
      invoke('get_pomodoro_sessions'),
      invoke('get_counters')  // Add counters to the data load
    ]);

    // Check if any data has actually changed
    const hasDataChanged = 
      JSON.stringify(todos) !== JSON.stringify(newTodos) ||
      JSON.stringify(goals) !== JSON.stringify(newGoals) ||
      JSON.stringify(notes) !== JSON.stringify(newNotes) ||
      JSON.stringify(pomodoroSessions) !== JSON.stringify(newPomodoroSessions) ||
      JSON.stringify(counters) !== JSON.stringify(newCounters);  // Add counters to hasDataChanged check

    // Update our data
    todos = newTodos;
    goals = newGoals;
    notes = newNotes;
    pomodoroSessions = newPomodoroSessions;  // Add this line
    counters = newCounters;
    
    // Sort and deduplicate journal entries
    allJournalEntries = newJournalEntries
      .sort((a: any, b: any) => new Date(b.date).getTime() - new Date(a.date).getTime())
      .filter((entry: any, index: number, self: any[]) => 
        index === self.findIndex((e: any) => 
          format(parseISO(e.date), 'yyyy-MM-dd') === format(parseISO(entry.date), 'yyyy-MM-dd')
        )
      );

    // Update completed todos for the selected date
    completedTodos = todos.filter(todo => 
      todo.completed && 
      format(parseISO(todo.updated_at), 'yyyy-MM-dd') === format(currentDate, 'yyyy-MM-dd')
    );

    // Get existing entry for the current date
    existingEntry = await invoke('get_journal_entry_for_date', { 
      date: format(currentDate, "yyyy-MM-dd'T'HH:mm:ss'Z'") 
    });

    // Only regenerate content if we're viewing today's entry and data has changed
    if (isToday(currentDate)) {
      if (hasDataChanged || !existingEntry) {
        // If there's an existing entry, preserve its reflections
        const existingReflections = existingEntry?.content?.match(/Personal reflections:\n([\s\S]*)/)?.[1]?.trim() || '';
        generateDefaultContent(existingReflections);
      } else if (!journalContent && existingEntry) {
        // If we don't have content but there's an existing entry, use it
        journalContent = existingEntry.content;
      }
    } else if (existingEntry) {
      // For past entries, always show the saved content
      journalContent = existingEntry.content;
    } else {
      // For new entries on past dates, generate fresh content
      generateDefaultContent('');
    }
  }

  function generateDefaultContent(existingReflections = '') {
    let content = `${formattedDate}\n\n`;
    const currentDateStr = formattedDate;

    // Get today's Pomodoro sessions
    const todaysPomodoroSessions = pomodoroSessions.filter(session => 
        format(parseISO(session.completed_at), 'yyyy-MM-dd') === currentDateStr
    );
    if (todaysPomodoroSessions.length > 0) {
      content += `Today's completed Pomodoro sessions:\n`;
      content += todaysPomodoroSessions.map(session => 
        `- ${session.session_name} (${session.work_minutes} minutes)`
      ).join('\n') + '\n\n';
    }

    // Add counter progress - exactly matching other trackers pattern
    const todaysCounters = counters.filter(counter => 
      format(parseISO(counter.updated_at), 'yyyy-MM-dd') === currentDateStr && 
      counter.value > 0  // Move this condition into the filter like other trackers
    );

    if (todaysCounters.length > 0) {
      content += `Progress tracked today:\n`;
      content += todaysCounters
        .map(counter => `- ${counter.name}: ${counter.value} times`)
        .join('\n') + '\n\n';
    }

    // Goals created today
    const todaysGoals = goals.filter(goal => 
      format(parseISO(goal.created_at), 'yyyy-MM-dd') === currentDateStr
    );
    if (todaysGoals.length > 0) {
      content += `Today I have set these new long term goals:\n`;
      content += todaysGoals.map(goal => `- ${goal.title}`).join('\n') + '\n\n';
    }

    // Todos created today
    const todaysTodos = todos.filter(todo => 
      !todo.completed && 
      format(parseISO(todo.created_at), 'yyyy-MM-dd') === currentDateStr
    );
    if (todaysTodos.length > 0) {
      content += `I have added these new to-dos:\n`;
      content += todaysTodos.map(todo => `- ${todo.text}`).join('\n') + '\n\n';
    }

    // Completed todos
    if (completedTodos.length > 0) {
      content += `I have completed these to-dos:\n`;
      content += completedTodos.map(todo => `- ${todo.text}`).join('\n') + '\n\n';
    }

    // Notes created today
    const todaysNotes = notes.filter(note => 
      format(parseISO(note.created_at), 'yyyy-MM-dd') === currentDateStr
    );
    if (todaysNotes.length > 0) {
      content += `I have added these new notes:\n`;
      content += todaysNotes.map(note => `- ${note.title}`).join('\n') + '\n\n';
    }

    // Notes updated today (but not created today)
    const updatedNotes = notes.filter(note => 
      format(parseISO(note.updated_at), 'yyyy-MM-dd') === currentDateStr &&
      format(parseISO(note.created_at), 'yyyy-MM-dd') !== currentDateStr
    );
    if (updatedNotes.length > 0) {
      content += `I have written more notes for:\n`;
      content += updatedNotes.map(note => `- ${note.title}`).join('\n') + '\n\n';
    }

    // Personal reflections should always be last
    content += `Personal reflections:\n${existingReflections}`;

    journalContent = content;
  }

  async function saveJournalEntry() {
    if (journalContent.trim()) {
      await invoke('add_journal_entry', {
        content: journalContent,
        date: format(currentDate, "yyyy-MM-dd'T'HH:mm:ss'Z'")
      });
      await loadData(); // Reload data to update the entries list
    }
  }

  async function handleDateChange() {
    currentDate = startOfDay(parseISO(selectedEntryDate));
    formattedDate = format(currentDate, 'yyyy-MM-dd');
    await loadData();
  }
</script>

<div class="journal-page">
  <h1>Journal</h1>
  <select bind:value={selectedEntryDate} on:change={handleDateChange}>
    <option value={format(new Date(), 'yyyy-MM-dd')}>Today</option>
    {#each allJournalEntries as entry}
      {#if !isToday(parseISO(entry.date))}
        <option value={format(parseISO(entry.date), 'yyyy-MM-dd')}>
          {format(parseISO(entry.date), 'yyyy-MM-dd')}
        </option>
      {/if}
    {/each}
  </select>

  <div class="notebook">
    <textarea
      bind:value={journalContent}
      rows="20"
      on:input={saveJournalEntry}
      placeholder="Write your journal entry here..."
      spellcheck="false"
    ></textarea>
  </div>
</div>

<style>
  .journal-page {
    max-width: 800px;
    margin: 0 auto;
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .notebook {
    background-color: var(--secondary-color);
    border: 1px solid var(--secondary-color);
    border-radius: 5px;
    padding: 20px;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    margin-top: 20px;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
  }

  textarea {
    width: 100%;
    border: none;
    background-color: transparent;
    font-size: 16px;
    line-height: 1.6;
    color: black;
    text-decoration: none;
    flex-grow: 1;
    resize: none;
  }

  textarea:focus {
    outline: none;
    text-decoration: none;
  }

  button {
    margin-top: 20px;
    padding: 10px 20px;
    background-color: var(--accent-color);
    color: var(--primary-color);
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
  }

  button:hover {
    background-color: var(--secondary-color);
  }

  select {
    width: 100%;
    padding: 10px;
    margin-bottom: 20px;
    font-size: 16px;
    border: 1px solid var(--secondary-color);
    border-radius: 5px;
    background-color: var(--primary-color);
    color: var(--text-color);
  }
</style>
