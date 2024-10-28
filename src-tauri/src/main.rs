// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use log::{info, error, debug};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use tauri::Manager;
use std::process::Command;

// Define the structure of a todo item
#[derive(Clone, Serialize, Deserialize, Debug)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Define the structure of a note
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Define the structure of a goal
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Goal {
    id: usize,
    title: String,
    motivation: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Define the structure of Pomodoro state
#[derive(Clone, Serialize, Deserialize, Debug)]
struct PomodoroState {
    session_name: String,
    current_mode: String,
    remaining_seconds: u32,
    is_running: bool,
    cycle_count: u32,
    start_time: u64,  // Added for tracking when timer started
}

// Define the structure of a journal entry
#[derive(Clone, Serialize, Deserialize, Debug)]
struct JournalEntry {
    id: usize,
    date: DateTime<Utc>,
    content: String,
}

// Define the structure of Pomodoro session
#[derive(Clone, Serialize, Deserialize, Debug)]
struct PomodoroSession {
    id: usize,
    session_name: String,
    work_minutes: u32,
    completed_at: DateTime<Utc>,
}

// Define the structure of app data
#[derive(Clone, Serialize, Deserialize, Debug)]
struct AppData {
    todos: Vec<TodoItem>,
    notes: Vec<Note>,
    goals: Vec<Goal>,
    pomodoro: PomodoroState,
    journal_entries: Vec<JournalEntry>,
    pomodoro_sessions: Vec<PomodoroSession>,  // Add this line
}

// Wrapper struct for a thread-safe AppData
struct AppState(Mutex<AppData>);



// Function to get the path where app data will be stored
fn get_data_file_path() -> PathBuf {
    let app_data_dir = tauri::api::path::app_data_dir(&tauri::Config::default()).expect("Failed to get app data dir");
    let path = app_data_dir.join("app_data.json");
    debug!("Data file path: {:?}", path);
    path
}

// Function to save app data to a file
fn save_app_data(app_data: &AppData) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(app_data)?;
    let path = get_data_file_path();
    fs::write(&path, &json)?;
    info!("Saved app data to {:?}", path);
    Ok(())
}

// Function to load app data from a file
fn load_app_data() -> Result<AppData, Box<dyn std::error::Error>> {
    let path = get_data_file_path();
    if path.exists() {
        let json = fs::read_to_string(&path)?;
        let app_data: AppData = serde_json::from_str(&json)?;
        Ok(app_data)
    } else {
        Ok(AppData {
            todos: Vec::new(),
            notes: Vec::new(),
            goals: Vec::new(),
            pomodoro: PomodoroState {
                session_name: String::new(),
                current_mode: "work".to_string(),
                remaining_seconds: 25 * 60,
                is_running: false,
                cycle_count: 0,
                start_time: 0,
            },
            journal_entries: Vec::new(),
            pomodoro_sessions: Vec::new(),  // Add this line
        })
    }
}

// Tauri command to add a new todo
#[tauri::command]
fn add_todo(state: State<AppState>, todo: String) -> Result<Vec<TodoItem>, String> {
    let mut app_data = state.0.lock().unwrap();
    let new_id = app_data.todos.len();
    let now = Utc::now();
    app_data.todos.push(TodoItem {
        id: new_id,
        text: todo.clone(),
        completed: false,
        created_at: now,
        updated_at: now,
    });
    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Added new todo: {}", todo);
            Ok(app_data.todos.clone())
        },
        Err(e) => {
            error!("Failed to save app data after adding todo: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to remove a todo
#[tauri::command]
fn remove_todo(state: State<AppState>, id: usize) -> Result<Vec<TodoItem>, String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.todos.retain(|todo| todo.id != id);
    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Removed todo with id: {}", id);
            Ok(app_data.todos.clone())
        },
        Err(e) => {
            error!("Failed to save app data after removing todo: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to toggle a todo's completed status
#[tauri::command]
fn toggle_todo(state: State<AppState>, id: usize) -> Result<Vec<TodoItem>, String> {
    let mut app_data = state.0.lock().unwrap();
    if let Some(todo) = app_data.todos.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        todo.updated_at = Utc::now();
        info!("Toggled todo with id: {}, new state: {}", id, todo.completed);
    } else {
        error!("Failed to find todo with id: {}", id);
    }
    match save_app_data(&app_data) {
        Ok(_) => Ok(app_data.todos.clone()),
        Err(e) => {
            error!("Failed to save app data after toggling todo: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to get all todos
#[tauri::command]
fn get_todos(state: State<AppState>) -> Vec<TodoItem> {
    let app_data = state.0.lock().unwrap();
    info!("Returning {} todos", app_data.todos.len());
    app_data.todos.clone()
}

// Tauri command to add a new note
#[tauri::command]
fn add_note(state: State<AppState>, title: String, content: String) -> Result<Vec<Note>, String> {
    let mut app_data = state.0.lock().unwrap();
    let new_id = app_data.notes.len();
    let now = Utc::now();
    app_data.notes.push(Note {
        id: new_id,
        title: title.clone(),
        content,
        created_at: now,
        updated_at: now,
    });
    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Added new note: {}", title);
            Ok(app_data.notes.clone())
        },
        Err(e) => {
            error!("Failed to save app data after adding note: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to update a note
#[tauri::command]
fn update_note(state: State<AppState>, id: usize, title: String, content: String) -> Result<Vec<Note>, String> {
    let mut app_data = state.0.lock().unwrap();
    if let Some(note) = app_data.notes.iter_mut().find(|n| n.id == id) {
        note.title = title;
        note.content = content;
        note.updated_at = Utc::now();
        match save_app_data(&app_data) {
            Ok(_) => {
                info!("Updated note with id: {}", id);
                Ok(app_data.notes.clone())
            },
            Err(e) => {
                error!("Failed to save app data after updating note: {}", e);
                Err(e.to_string())
            }
        }
    } else {
        Err("Note not found".to_string())
    }
}

// Tauri command to delete a note
#[tauri::command]
fn delete_note(state: State<AppState>, id: usize) -> Result<Vec<Note>, String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.notes.retain(|note| note.id != id);
    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Deleted note with id: {}", id);
            Ok(app_data.notes.clone())
        },
        Err(e) => {
            error!("Failed to save app data after deleting note: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to get all notes
#[tauri::command]
fn get_notes(state: State<AppState>) -> Vec<Note> {
    let app_data = state.0.lock().unwrap();
    info!("Returning {} notes", app_data.notes.len());
    app_data.notes.clone()
}

// Tauri command to add a new goal
#[tauri::command]
fn add_goal(state: State<AppState>, title: String, motivation: String) -> Result<Vec<Goal>, String> {
    let mut app_data = state.0.lock().unwrap();
    let new_id = app_data.goals.len();
    let now = Utc::now();
    app_data.goals.push(Goal {
        id: new_id,
        title: title.clone(),
        motivation,
        created_at: now,
        updated_at: now,
    });
    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Added new goal: {}", title);
            Ok(app_data.goals.clone())
        },
        Err(e) => {
            error!("Failed to save app data after adding goal: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to remove a goal
#[tauri::command]
fn remove_goal(state: State<AppState>, id: usize) -> Result<Vec<Goal>, String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.goals.retain(|goal| goal.id != id);
    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Removed goal with id: {}", id);
            Ok(app_data.goals.clone())
        },
        Err(e) => {
            error!("Failed to save app data after removing goal: {}", e);
            Err(e.to_string())
        }
    }
}

// Tauri command to update a goal
#[tauri::command]
fn update_goal(state: State<AppState>, id: usize, title: String, motivation: String) -> Result<Vec<Goal>, String> {
    let mut app_data = state.0.lock().unwrap();
    if let Some(goal) = app_data.goals.iter_mut().find(|g| g.id == id) {
        goal.title = title;
        goal.motivation = motivation;
        goal.updated_at = Utc::now();
        match save_app_data(&app_data) {
            Ok(_) => {
                info!("Updated goal with id: {}", id);
                Ok(app_data.goals.clone())
            },
            Err(e) => {
                error!("Failed to save app data after updating goal: {}", e);
                Err(e.to_string())
            }
        }
    } else {
        Err("Goal not found".to_string())
    }
}

// Tauri command to get all goals
#[tauri::command]
fn get_goals(state: State<AppState>) -> Vec<Goal> {
    let app_data = state.0.lock().unwrap();
    info!("Returning {} goals", app_data.goals.len());
    app_data.goals.clone()
}

// Tauri command to save Pomodoro state
#[tauri::command]
fn save_pomodoro_state(state: State<AppState>, pomodoro_state: PomodoroState) -> Result<(), String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.pomodoro = PomodoroState {
        session_name: pomodoro_state.session_name,
        current_mode: pomodoro_state.current_mode,
        remaining_seconds: pomodoro_state.remaining_seconds,
        is_running: pomodoro_state.is_running,
        cycle_count: pomodoro_state.cycle_count,
        start_time: pomodoro_state.start_time,
    };
    save_app_data(&app_data).map_err(|e| e.to_string())
}

// Tauri command to get Pomodoro state
#[tauri::command]
fn get_pomodoro_state(state: State<AppState>) -> PomodoroState {
    let app_data = state.0.lock().unwrap();
    app_data.pomodoro.clone()
}


#[tauri::command]
fn start_pomodoro(state: State<AppState>) -> Result<PomodoroState, String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.pomodoro.is_running = true;
    app_data.pomodoro.start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    save_app_data(&app_data).map_err(|e| e.to_string())?;
    Ok(app_data.pomodoro.clone())
}


#[tauri::command]
fn stop_pomodoro(state: State<AppState>) -> Result<PomodoroState, String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.pomodoro.is_running = false;
    save_app_data(&app_data).map_err(|e| e.to_string())?;
    Ok(app_data.pomodoro.clone())
}

#[tauri::command]
fn reset_pomodoro(state: State<AppState>) -> Result<PomodoroState, String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.pomodoro = PomodoroState {
        session_name: String::new(),
        current_mode: "work".to_string(),
        remaining_seconds: 25 * 60, // 25 minutes in seconds
        is_running: false,
        cycle_count: 0,
        start_time: 0,
    };
    
    save_app_data(&app_data).map_err(|e| e.to_string())?;
    Ok(app_data.pomodoro.clone())
}





#[tauri::command]
fn get_data_file_path_string() -> String {
    get_data_file_path().to_str().unwrap_or("").to_string()
}

#[tauri::command]
fn open_user_data_file(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = get_data_file_path();
    let path_str = path.to_str().unwrap_or("").to_string();
    
    if path.exists() {
        #[cfg(target_os = "windows")]
        {
            match Command::new("cmd").args(&["/C", "start", "", &path_str]).output() {
                Ok(_) => Ok(format!("Attempted to open file: {}", path_str)),
                Err(e) => Err(format!("Failed to open file: {}. Error: {}", path_str, e)),
            }
        }
        #[cfg(target_os = "macos")]
        {
            match Command::new("open").arg(&path_str).output() {
                Ok(_) => Ok(format!("Attempted to open file: {}", path_str)),
                Err(e) => Err(format!("Failed to open file: {}. Error: {}", path_str, e)),
            }
        }
        #[cfg(target_os = "linux")]
        {
            match Command::new("xdg-open").arg(&path_str).output() {
                Ok(_) => Ok(format!("Attempted to open file: {}", path_str)),
                Err(e) => Err(format!("Failed to open file: {}. Error: {}", path_str, e)),
            }
        }
    } else {
        Err(format!("User data file does not exist at: {}", path_str))
    }
}

// Modify the add_journal_entry function
#[tauri::command]
fn add_journal_entry(state: State<AppState>, content: String, date: String) -> Result<Vec<JournalEntry>, String> {
    let mut app_data = state.0.lock().unwrap();
    let target_date = DateTime::parse_from_rfc3339(&date).unwrap().with_timezone(&Utc);
    
    // Check if an entry for this date already exists
    if let Some(existing_entry) = app_data.journal_entries.iter_mut().find(|entry| entry.date.date() == target_date.date()) {
        // Update existing entry
        existing_entry.content = content;
        existing_entry.date = target_date;
    } else {
        // Create new entry
        let new_id = app_data.journal_entries.len();
        app_data.journal_entries.push(JournalEntry {
            id: new_id,
            date: target_date,
            content,
        });
    }

    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Added or updated journal entry for {}", date);
            Ok(app_data.journal_entries.clone())
        },
        Err(e) => {
            error!("Failed to save app data after adding/updating journal entry: {}", e);
            Err(e.to_string())
        }
    }
}

// Modify the get_journal_entries function to sort entries by date
#[tauri::command]
fn get_journal_entries(state: State<AppState>) -> Vec<JournalEntry> {
    let mut app_data = state.0.lock().unwrap();
    
    // Remove duplicates by keeping only the latest entry for each date
    let mut unique_entries: Vec<JournalEntry> = Vec::new();
    for entry in app_data.journal_entries.iter() {
        if !unique_entries.iter().any(|e| e.date.date() == entry.date.date()) {
            unique_entries.push(entry.clone());
        }
    }
    
    // Sort entries by date (newest first)
    unique_entries.sort_by(|a, b| b.date.cmp(&a.date));
    unique_entries
}

#[tauri::command]
fn get_journal_entry_for_date(state: State<AppState>, date: String) -> Option<JournalEntry> {
    let app_data = state.0.lock().unwrap();
    let target_date = DateTime::parse_from_rfc3339(&date).unwrap().with_timezone(&Utc);
    app_data.journal_entries.iter().find(|entry| entry.date.date() == target_date.date()).cloned()
}

// Add a new function to update the Pomodoro state
#[tauri::command]
fn update_pomodoro_state(state: State<AppState>, pomodoro_state: PomodoroState) -> Result<(), String> {
    let mut app_data = state.0.lock().unwrap();
    app_data.pomodoro = pomodoro_state;
    save_app_data(&app_data).map_err(|e| e.to_string())
}

// Add new command to save completed session
#[tauri::command]
fn save_pomodoro_session(state: State<AppState>, session_name: String, work_minutes: u32) -> Result<Vec<PomodoroSession>, String> {
    let mut app_data = state.0.lock().unwrap();
    let new_id = app_data.pomodoro_sessions.len();
    
    app_data.pomodoro_sessions.push(PomodoroSession {
        id: new_id,
        session_name,
        work_minutes,
        completed_at: Utc::now(),
    });

    match save_app_data(&app_data) {
        Ok(_) => {
            info!("Saved pomodoro session: {} minutes", work_minutes);
            Ok(app_data.pomodoro_sessions.clone())
        },
        Err(e) => {
            error!("Failed to save pomodoro session: {}", e);
            Err(e.to_string())
        }
    }
}

// Add getter for pomodoro history
#[tauri::command]
fn get_pomodoro_sessions(state: State<AppState>) -> Vec<PomodoroSession> {
    let app_data = state.0.lock().unwrap();
    app_data.pomodoro_sessions.clone()
}

fn main() {
    // Initialize logger for debug builds
    #[cfg(debug_assertions)]
    {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
            .init();
    }

    // Initialize logger for release builds
    #[cfg(not(debug_assertions))]
    {
        env_logger::init();
    }

    info!("Starting application");

    // Load existing app data or create new with defaults
    let app_data = load_app_data().unwrap_or_else(|e| {
        error!("Failed to load app data: {}", e);
        AppData {
            todos: Vec::new(),
            notes: Vec::new(),
            goals: Vec::new(),
            pomodoro: PomodoroState {
                session_name: String::new(),
                current_mode: "work".to_string(),
                remaining_seconds: 25 * 60,
                is_running: false,
                cycle_count: 0,
                start_time: 0,
            },
            journal_entries: Vec::new(),
            pomodoro_sessions: Vec::new(),  // Add this line
        }
    });

    // Build and run the Tauri application
    tauri::Builder::default()
        .manage(AppState(Mutex::new(app_data)))
        .invoke_handler(tauri::generate_handler![
            add_todo, remove_todo, toggle_todo, get_todos,
            add_note, update_note, delete_note, get_notes,
            add_goal, remove_goal, update_goal, get_goals,
            get_pomodoro_state, update_pomodoro_state, 
            start_pomodoro, stop_pomodoro, reset_pomodoro,
            save_pomodoro_session, get_pomodoro_sessions,
            get_data_file_path_string, open_user_data_file,
            add_journal_entry,
            get_journal_entries,
            get_journal_entry_for_date,
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
