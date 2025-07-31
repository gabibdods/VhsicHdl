# VhsicHdl

# A Modern Rust-Based VHDL Integrated Development Environment

### Description

- This project delivers a full-featured VHDL IDE tailored for hardware engineers and digital designers
- Built using Rust and GTK, it offers a cohesive development workflow with integrated simulation via GHDL, waveform inspection via GTKWave, and an extensible interface resembling the sophistication of JetBrains IDEs

---

## NOTICE

- Please read through this `README.md` to better understand the project's source code and setup instructions
- Also, make sure to review the contents of the `License/` directory
- Your attention to these details is appreciated — enjoy exploring the project!

---

## Problem Statement

- VHDL developers often rely on disjointed toolchains, limited syntax highlighting, and outdated UIs
- This project addresses the absence of a modern, all-in-one desktop IDE for VHDL that offers simulation, debugging, waveform analysis, and a refined GUI in a single package

---

## Project Goals

### Build an Integrated Simulation and Development Environment

- Offer GHDL-based simulation and automatic waveform visualization within a Rust-powered GTK interface

### Improve the Development Experience for VHDL

- Provide modern IDE features such as syntax highlighting, tabbed editing, autosave, project navigation, and support for external tools like Vivado and ModelSim

---

## Tools, Materials & Resources

### GUI Framework

- GTK 4 with gtk-rs bindings, allowing for rich UI design in Rust

### Simulation Tools

- GHDL (open-source simulator) and GTKWave for waveform analysis

### Development Tools

- GtkSourceView 5 for code editing, and Cargo for Rust-based builds

---

## Design Decision

### Rust + GTK as Core Stack

- Chosen for memory safety, cross-platform GUI capabilities, and strong ecosystem support for embedded systems development

### Modular Editor Design

- Each tab maintains an independent buffer and save state, enabling parallel editing without conflicts

### Embedded Simulation Output

- GHDL output is captured and displayed in a built-in terminal, streamlining the debugging process

---

## Features

### Syntax-Aware Editing

- Full VHDL syntax highlighting, autosave support, `Save As`, and tabbed navigation

### Integrated Simulation with GHDL

- Supports analysis, elaboration, and execution from within the IDE with top-level entity selection

### One-Click GTKWave Launch

- Automatically opens the simulation result (`.vcd`) in GTKWave post-run

---

## Block Diagram

|GTK Application           |Integrations                   |
|--------------------------|-------------------------------|
| MenuBar                  | Simulation Controls           |
| Notebook (tabs)          | Entity/Testbench Selector     |
| GtkSourceView Buffers    | GHDL Process Management       |
| Save, Open, Autosave     | GTKWave Launcher              |
|--------------------------|-------------------------------|
| Built-in Output Terminal (TextView + Buffer)             |
|----------------------------------------------------------|

---

## Functional Overview

- Startup initializes layout and registers buffer tracking
- Tabs encapsulate file I/O and simulation configuration per file
- GHDL is invoked via CLI for analysis/elaboration/simulation
- Waveform output is detected and passed to GTKWave
- Output logs are piped directly into the UI

---

## Challenges & Solutions

### Tab-Specific State Management

- Used `Rc<RefCell<...>>` to safely share mutable state across tabs and simulation logic

### GHDL CLI Integration

- Developed a Rust wrapper for ghdl commands with robust output capturing and error management

---

## Lessons Learned

### Ownership Model Benefits

- Rust's ownership and borrowing system made UI lifetime management and buffer safety explicit and reliable

### VHDL Toolchain Complexity

- Gained deep insight into the sequencing of VHDL simulation steps and the handling of testbench hierarchies

---

## Project Structure

```plaintext
root/
├── License/
│   ├── LICENSE.md
│   │
│   └── NOTICE.md
│
├── .gitattributes
│
├── .gitignore
│
├── README.md
│
├── build/
│   ├── ico #Icon
│   │
│   ├── logo #Logo
│   │
│   └── windows #Windows 10 logo
│
└── frontend/
    ├── src/
    │   ├── ghdl_runner.rs
    │   │
    │   └── main.rs
    │
    └── Cargo.toml

```

---

## Future Enhancements

- Add GHDL formatting with `ghdl fmt`
- Annotate errors with inline markers and navigable diagnostics
- Introduce project workspace configuration (via XML or TOML)
- Support synthesis via Vivado and ModelSim
- Embed waveform viewer using GTK3 interop
- Enable code completion using a VHDL-specific LSP backend


