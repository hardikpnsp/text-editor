## Description

A terminal app for text editing using Rust

## Run
- clone repo
- run: `cargo run` to open the terminal editor
- run: `cargo test` to run test cases

## Usage
- Ctrl + N: open new file to edit
- Ctrl + R: to rotate between open files
- Ctrl + S: Save current file
- Esc: Exit current file
- Arrow Keys: cursor movement
- Backspace: erase character

## Notes

- If you are looking for just the features implemented during hackathon, checkout to `hackathon` tag.

### Level 1: MVP

- A command line utility: `te`
- open an existing file using `te <file-name>`
  - a "text-area" in terminal with content of the file is displayed
- make changes to the opened file
  - basic cursor navigation with arrow keys
  - backspace to erase content 
- save the changes (with a key combination like `Ctrl + S`)
- escape to exit

### Libraries for TUI

- tui-rs with tui-textarea (third party widget)
  - high level abstraction
- termbox (rust wrapper -> rustbox)
  - minimalist 
  - viewing terminals as a table of fix sized cells
  - input is a stream of structured messages
- termion (alternative to termbox)
  - low level control
  - can handle cursor movement, text formatting

Choosing termion as the TUI library 

### Progress so far

- [x] open and display file
- [x] edit file
  - [x] cursor movement
    - [x] cursor should move with to arrow keys
  - [x] user input
    - [x] Enter for next line
    - [x] Backspace to delete character
    - [x] Ctrl + s to save file
    - [x] Chars should be written where cursor is
- [x] save file

- [x] text wrapping
- [x] opening and editing multiple files
- [x] add row:col at the bottom right to show cursor position
- [ ] memory optimization
