<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { flip } from 'svelte/animate';
    import { fade, fly } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { format, parseISO } from 'date-fns';
  
    interface TodoItem {
      id: number;
      text: string;
      completed: boolean;
      created_at: string;
    }
  
    let newTodo = "";
    let todos: TodoItem[] = [];
    let error = "";
  
    async function addTodo() {
      if (newTodo.trim()) {
        try {
          todos = await invoke("add_todo", { todo: newTodo });
          newTodo = "";
          error = "";
        } catch (e) {
          error = `Failed to add todo: ${e}`;
        }
      }
    }
  
    async function removeTodo(id: number) {
      try {
        todos = await invoke("remove_todo", { id });
        error = "";
      } catch (e) {
        error = `Failed to remove todo: ${e}`;
      }
    }
  
    async function toggleTodo(id: number) {
      try {
        todos = await invoke("toggle_todo", { id });
        error = "";
      } catch (e) {
        error = `Failed to toggle todo: ${e}`;
      }
    }
  
    async function loadTodos() {
      try {
        todos = await invoke("get_todos");
        error = "";
      } catch (e) {
        error = `Failed to load todos: ${e}`;
      }
    }
  
    onMount(loadTodos);
  </script>

<div class="container">
  <h1>Todo List</h1>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <form class="add-todo-form" on:submit|preventDefault={addTodo}>
    <input
      id="todo-input"
      placeholder="Enter a new todo..."
      bind:value={newTodo}
    />
    <button type="submit">Add Todo</button>
  </form>

  <ul class="todo-list">
    {#each todos as todo (todo.id)}
      <li 
        class="todo-item" 
        class:completed={todo.completed}
        animate:flip={{ duration: 300 }}
        in:fly={{ y: 20, duration: 300 }}
        out:fade={{ duration: 300 }}
      >
        <label class="checkbox-container">
          <input
            type="checkbox"
            checked={todo.completed}
            on:change={() => toggleTodo(todo.id)}
          />
          <span class="checkmark"></span>
        </label>
        <span class="todo-text">{todo.text}</span>
        <span class="date-text" data-full-date={format(parseISO(todo.created_at), 'MMM d, yyyy h:mm a')}>
          <span class="hover-date">{format(parseISO(todo.created_at), 'MMM d, yyyy h:mm a')}</span>
        </span>
        <button class="remove-btn" on:click={() => removeTodo(todo.id)}>
          <span class="remove-icon">×</span>
        </button>
      </li>
    {/each}
  </ul>
</div>

<style>
  .container {
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    color: var(--secondary-color);
    text-align: center;
    font-weight: 600;
    font-size: 2.5rem;
    margin-bottom: 1.5rem;
  }

  .add-todo-form {
    display: flex;
    margin-bottom: 1rem;
  }

  #todo-input {
    flex-grow: 1;
    padding: 0.5rem;
    font-size: 1rem;
    font-family: var(--font-main);
    border: 1px solid var(--accent-color);
    border-radius: 4px 0 0 4px;
    background-color: var(--secondary-color);
    color: black;
  }

  button {
    padding: 0.5rem 1rem;
    font-size: 1rem;
    font-family: var(--font-main);
    font-weight: 700;
    background-color: var(--secondary-color);
    color: var(--primary-color);
    border: none;
    cursor: pointer;
    transition: background-color 0.3s, color 0.3s;
  }

  button:hover {
    background-color: var(--accent-color);
    color: var(--secondary-color);
  }

  .todo-list {
    list-style-type: none;
    padding: 0;
  }

  .todo-item {
    display: flex;
    align-items: center;
    padding: 0.5rem;
    background-color: var(--primary-color);
    border: 1px solid var(--accent-color);
    border-radius: 4px;
    margin-bottom: 0.5rem;
    transition: all 0.3s;
    position: relative;
  }

  .todo-item:hover {
    background-color: var(--secondary-color);
    color: black;
  }

  .todo-item:hover .remove-btn {
    color: black;
  }

  .todo-text {
    flex-grow: 1;
    margin-left: 0.5rem;
    font-size: 1rem;
  }

  .completed .todo-text {
    text-decoration: line-through;
  }

  .remove-btn {
    background-color: transparent;
    color: var(--secondary-color);
    border: none;
    cursor: pointer;
    font-size: 1.2rem;
    padding: 0 0.5rem;
  }

  .remove-btn:hover {
    color: #ff4136;
  }

  .checkbox-container {
    display: flex;
    align-items: center;
    position: relative;
    padding-left: 35px;
    cursor: pointer;
    font-size: 22px;
    user-select: none;
  }

  .checkbox-container input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0;
  }

  .checkmark {
    position: absolute;
    top: 50%;
    left: 0;
    transform: translateY(-50%);
    height: 25px;
    width: 25px;
    background-color: var(--primary-color);
    border: 2px solid var(--secondary-color);
    border-radius: 4px;
  }

  .checkbox-container:hover input ~ .checkmark {
    background-color: var(--accent-color);
  }

  .checkbox-container input:checked ~ .checkmark {
    background-color: var(--secondary-color);
  }

  .checkmark:after {
    content: "";
    position: absolute;
    display: none;
  }

  .checkbox-container input:checked ~ .checkmark:after {
    display: block;
  }

  .checkbox-container .checkmark:after {
    left: 9px;
    top: 5px;
    width: 5px;
    height: 10px;
    border: solid var(--primary-color);
    border-width: 0 3px 3px 0;
    transform: rotate(45deg);
  }

  .error {
    color: #ff4136;
    background-color: #ffeeee;
    border: 1px solid #ff4136;
    border-radius: 4px;
    padding: 0.5rem;
    margin-bottom: 1rem;
  }

  .date-text {
    position: relative;
    margin-right: 0.5rem;
    display: flex;
    align-items: center;
    height: 100%;
  }

  .hover-date {
    position: absolute;
    right: 0;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.3s, visibility 0.3s;
    color: black;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.8rem;
    white-space: nowrap;
    top: 50%;
    transform: translateY(-50%);
  }

  .todo-item:hover .hover-date {
    opacity: 1;
    visibility: visible;
  }

  .todo-item:hover .date-text {
    color: var(--primary-color);
  }
</style>
