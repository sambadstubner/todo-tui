# Todo TUI

A Terminal User Interface (TUI) todo application built in Rust, using CSV files for data storage. Features vim-like navigation and a clean, efficient interface.

## Features

### ✅ Implemented Features

- **Task Management**: Create, edit, delete, and toggle completion of tasks
- **List Management**: Organize tasks into different lists
- **Due Dates**: Set due dates with smart parsing (supports "today", "tomorrow", "YYYY-MM-DD" format)
- **Recurring Tasks**: Set frequency for recurring tasks (daily, weekdays, weekly, monthly, yearly)
- **My Day**: View and manage tasks for today, with manual addition/removal
- **Notes**: Add optional notes to tasks and view them
- **Task Movement**: Move tasks between different lists
- **CSV Storage**: All data is stored in CSV files for easy backup and portability
- **Completed Tasks**: Completed tasks are shown separately at the bottom of each list
- **Visual Indicators**: 
  - Overdue tasks shown in red
  - Due today tasks shown in yellow
  - My Day tasks marked with ⭐
  - Recurring frequency displayed as [Daily], [Weekly], etc.

### 🎯 Key Features

- **Multi-step Task Creation**: When creating or editing tasks, you'll be guided through:
  1. Task title (required)
  2. Due date (optional, supports relative dates)
  3. Recurring frequency (optional)
  4. Notes (optional)

- **Smart Date Parsing**: Supports various date formats:
  - Relative: "today", "tomorrow", "next week", "next month"
  - Absolute: "2024-01-15", "01/15/2024", "15/01/2024"

- **Task Organization**: 
  - Active tasks shown first
  - Completed tasks shown at the bottom with a separator
  - Task counts displayed in list titles

## Installation

1. Clone the repository
2. Build with Cargo:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
cargo run
```

### Keyboard Shortcuts

#### General Navigation
- `Ctrl+Q`: Quit application
- `Esc`: Go back/cancel
- `↑/↓` or `j/k`: Navigate lists and tasks (vim-like)
- `gg`: Jump to top of list
- `G`: Jump to bottom of list
- `Enter`: Select/edit item

#### Task Management
- `Ctrl+N`: Create new task
- `Ctrl+E`: Edit selected task
- `Ctrl+V`: View task notes
- `Ctrl+T`: Move task to another list
- `Space`: Toggle task completion
- `Del/Backspace`: Delete selected task
- `Ctrl+D`: Add/remove task from "My Day"

#### Navigation
- `Ctrl+M`: Go to "My Day" view
- `j/k`: Move up/down (vim-like navigation)
- `Tab`: Switch between lists (when in list overview)

### Task Editor

When creating or editing a task, you'll go through these steps:

1. **Title**: Enter the task title (required)
2. **Due Date**: Enter due date or press Enter to skip
3. **Frequency**: Enter recurring frequency or press Enter to skip
4. **Notes**: Enter optional notes or press Enter to finish

## Data Storage

All data is stored in CSV files in the `~/todo-data/` directory (in your home folder):
- `tasks.csv`: All task data including titles, due dates, frequencies, notes, and completion status
- `lists.csv`: List information

The data directory is automatically created when you first run the application, ensuring your todos persist regardless of where you run the application from.

The CSV format makes it easy to:
- Backup your data
- Import/export tasks
- View data in spreadsheet applications
- Integrate with other tools

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── app.rs                  # Main application state
├── models/
│   ├── task.rs            # Task data model
│   ├── list.rs            # List data model
│   └── storage.rs         # CSV storage operations
├── ui/
│   ├── mod.rs             # Main UI coordinator
│   └── screens/           # Different UI screens
├── handlers/
│   └── input.rs           # Keyboard input handling
└── utils/
    ├── date_utils.rs      # Date parsing utilities
    └── validation.rs      # Input validation
```

## Dependencies

- `ratatui`: Modern TUI framework
- `crossterm`: Cross-platform terminal handling
- `chrono`: Date and time handling
- `csv`: CSV file operations
- `serde`: Data serialization
- `anyhow`: Error handling

## Future Enhancements

Potential features for future versions:
- Task search and filtering
- Task priorities
- Subtasks
- Task templates
- Export to other formats
- Themes and customization
- Keyboard shortcuts customization
