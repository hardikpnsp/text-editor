## Description

A terminal app for text editing using Rust

## Level 1: MVP

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

# Progress so far

## Features
- [x] open and display file
- [ ] edit file
  - [ ] cursor movement
    - [x] cursor should move with to arrow keys
      - [ ] when pressed down/up, if the line length is shorter, cursor should move to end of line
  - [x] user input
    - [x] Enter for next line
    - [x] Backspace to delete character
    - [x] Ctrl + s to save file
    - [x] Chars should be written where cursor is
- [x] save file