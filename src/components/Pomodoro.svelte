<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  // Fixed durations (in seconds)
  const WORK_DURATION = 25 * 60;
  const SHORT_BREAK = 5 * 60;
  const LONG_BREAK = 15 * 60;

  // Add sound effect
  const dingSound = new Audio('sounds/ding.mp3');

  let remainingSeconds = WORK_DURATION;
  let isRunning = false;
  let cycleCount = 0;
  let currentMode = 'work';
  let sessionName = '';
  

  onMount(async () => {
    // Load initial state
    const state: any = await invoke('get_pomodoro_state');
    remainingSeconds = state.remaining_seconds;
    isRunning = state.is_running;
    cycleCount = state.cycle_count;
    currentMode = state.current_mode;
    sessionName = state.session_name;

    if (isRunning) {
      updateTimer();
    }
  });

  async function startTimer() {
    if (!isRunning) {
      await invoke('start_pomodoro');
      isRunning = true;
      dingSound.play(); // Play sound when timer starts
      updateTimer();
    }
  }

  async function stopTimer() {
    if (isRunning) {
      await invoke('stop_pomodoro');
      isRunning = false;
    }
  }

  async function resetTimer() {
    const state: any = await invoke('reset_pomodoro');
    remainingSeconds = state.remaining_seconds;
    isRunning = state.is_running;
    cycleCount = state.cycle_count;
    currentMode = state.current_mode;
    sessionName = state.session_name;
  }

  async function updateTimer() {
    if (isRunning && remainingSeconds > 0) {  // Only update if we have time remaining
      remainingSeconds--;
      if (remainingSeconds <= 0) {
        await handleTimerComplete();
      } else {
        setTimeout(updateTimer, 1000);
        await updatePomodoroState();
      }
    }
  }

  async function handleTimerComplete() {
    if (!isRunning) return;  // Prevent multiple calls
    
    isRunning = false;
    dingSound.play(); // Play sound when timer completes
    
    if (currentMode === 'work') {
      // Save completed work session
      if (sessionName.trim()) {  // Only save if there's a session name
        await invoke('save_pomodoro_session', {
          sessionName: sessionName,
          workMinutes: WORK_DURATION / 60
        });
      }
      
      cycleCount++;
      currentMode = cycleCount % 4 === 0 ? 'long-break' : 'short-break';
      remainingSeconds = cycleCount % 4 === 0 ? LONG_BREAK : SHORT_BREAK;
    } else {
      currentMode = 'work';
      remainingSeconds = WORK_DURATION;
    }
    await updatePomodoroState();
  }

  async function updatePomodoroState() {
    await invoke('update_pomodoro_state', {
      pomodoroState: {
        session_name: sessionName,
        current_mode: currentMode,
        remaining_seconds: remainingSeconds,
        is_running: isRunning,
        cycle_count: cycleCount,
        start_time: Date.now()
      }
    });
  }

  function formatTime(seconds: number): string {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`;
  }
</script>

<div class="pomodoro-container">
  <h2>Pomodoro Timer</h2>
  
  <div class="session-name-input">
    <label for="session-name">Session Name:</label>
    <input 
      type="text" 
      id="session-name" 
      bind:value={sessionName} 
      on:input={updatePomodoroState}
      placeholder="Enter session name"
      disabled={isRunning}
    >
  </div>

  <div class="timer">
    {formatTime(remainingSeconds)}
  </div>
  <div class="mode">
    Current Mode: {currentMode}
  </div>
  <div class="cycle">
    Cycle: {cycleCount}
  </div>
  <div class="controls">
    {#if !isRunning}
      <button on:click={startTimer}>Start</button>
    {:else}
      <button on:click={stopTimer}>Stop</button>
    {/if}
    <button 
      on:click={resetTimer} 
      class="reset-button"
    >
      Reset
    </button>
  </div>
</div>

<style>
  .pomodoro-container {
    background-color: var(--secondary-color);
    color: var(--primary-color);
    padding: 2rem;
    border-radius: 10px;
    text-align: center;
    max-width: 300px;
    margin: 0 auto;
  }

  h2 {
    margin-top: 0;
    font-weight: 600;
  }

  .timer {
    font-size: 3rem;
    font-weight: 600;
    margin: 1rem 0;
  }

  .mode, .cycle {
    font-size: 1rem;
    margin-bottom: 0.5rem;
  }

  .controls {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  button {
    background-color: var(--accent-color);
    color: var(--secondary-color);
    border: none;
    padding: 0.5rem 1rem;
    font-size: 1rem;
    cursor: pointer;
    border-radius: 5px;
    min-width: 80px;
  }

  button:hover {
    opacity: 0.9;
  }

  .reset-button {
    background-color: var(--warning-color, #e74c3c);
  }

  .session-name-input {
    margin-bottom: 1rem;
  }

  .session-name-input input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid var(--primary-color);
    border-radius: 3px;
    font-size: 1rem;
  }
</style>
