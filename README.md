# Mindflow

A minimalist personal productivity companion that helps you track habits, manage tasks, and maintain focus - all while keeping your data local and private.

This is a Work in Progress project, set to change a lot.

## Features

- **📝 Journal**: Automatic daily entries tracking your accomplishments and reflections
- **✅ Task Management**: Simple to-do list to track daily tasks
- **⏱️ Pomodoro Timer**: Stay focused with customizable work/break sessions
- **🎯 Goal Setting**: Track long-term goals and motivations
- **📔 Notes**: Quick note-taking for ideas and thoughts
- **🔒 Privacy-First**: All data stored locally in a single file

## Technology

Built with:
- [Tauri](https://tauri.app/) - Lightweight, secure desktop application framework
- [Svelte](https://svelte.dev/) - Efficient, reactive UI framework
- [TypeScript](https://www.typescriptlang.org/) - Type-safe JavaScript
- [Rust](https://www.rust-lang.org/) - Performance and reliability for the backend

## Getting Started

### Prerequisites
- [Node.js](https://nodejs.org/) (v16 or later)
- [Rust](https://www.rust-lang.org/tools/install)
- [VS Code](https://code.visualstudio.com/) (recommended)

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/forever8896/mindflow.git
   cd mindflow
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

To create a production build:
   ```bash
   npm run tauri build
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. This is still a simple work in progress aimed for personal use and helping me understand rust and tauri.

## License

[MIT License](LICENSE)

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- UI powered by [Svelte](https://svelte.dev/)
- Inspired by the need for a simple, private productivity tool
