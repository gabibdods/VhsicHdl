# ⚡ VHDL IDE

An integrated, modern, and powerful **VHDL IDE** designed to provide a complete development and simulation experience — **from zero to full verification** — all within one application.

---

## 🧠 Problem Statement

There is a distinct lack of **fully integrated VHDL IDEs** with modern GUI capabilities that resemble the completeness and developer experience of JetBrains tools (like CLion or IntelliJ IDEA). Existing options often rely on disjointed toolchains, command-line workflows, or limited editing capabilities.

---

## 🎯 Project Goal

Build a fully featured **desktop VHDL IDE** that supports the entire development lifecycle:
- Syntax highlighting, formatting, and navigation
- Simulation with **GHDL**
- Waveform analysis with **GTKWave**
- Project-based workflow (files, tabs, autosaving)
- Future integration with synthesis tools like **Vivado** and **ModelSim**

All of this, with the UI polish and usability of modern IDEs.

---

## 🛠️ Tech Stack

| Component           | Tool/Library               |
|---------------------|----------------------------|
| **GUI Toolkit**     | GTK 4 with `gtk-rs`        |
| **Editor**          | GtkSourceView 5            |
| **Language**        | Rust                       |
| **Simulation**      | GHDL                       |
| **Waveform Viewer** | GTKWave                    |
| **Build System**    | Cargo                      |
| **Version Control** | Git                        |

---

## 🧱 Design Decisions

- **GTK + Rust**: Offers memory safety, performance, and a rich UI ecosystem. Rust + GTK bindings are stable and production-ready.
- **GtkSourceView**: Enables powerful VHDL syntax highlighting and navigation.
- **Notebook (tabs)**: Independent buffer management and saving for each file.
- **Auto-launch GTKWave**: Instant waveform inspection post-simulation.
- **Built-in simulation panel**: Stream GHDL output directly into the IDE.

---

## 🧪 Current Features

- [x] Multi-tab source editing
- [x] Syntax highlighting for VHDL
- [x] Autosave & manual save
- [x] `Save As` and file selection dialogs
- [x] GHDL integration:
    - Analysis, Elaboration, and Simulation
    - Top-level entity/testbench selection
- [x] Output terminal panel
- [x] Auto-launch **GTKWave**
- [x] Proper `.gitignore`

---

## 🏗️ Architecture Overview

```text
+----------------------------------------------------------+
|                      GTK Application                     |
+--------------------------+-------------------------------+
| MenuBar                  | Simulation Controls           |
| Notebook (tabs)          | Entity/Testbench Selector     |
| GtkSourceView Buffers    | GHDL Process Management       |
| Save, Open, Autosave     | GTKWave Launcher              |
+--------------------------+-------------------------------+
| Built-in Output Terminal (TextView + Buffer)             |
+----------------------------------------------------------+

```

---

## 🛠️ Components:
- main.rs – App initialization and layout
- ghdl_runner.rs – CLI wrapper for GHDL tools
- tab.rs – Tab state (buffer, file path, etc.)
- autosave.rs – Background task for saving all tabs
- simulation.rs – Simulation execution logic
- gtkwave.rs – GTKWave auto-launcher

---

## 📚 Lessons Learned
- Rust's ownership model enforces clean UI lifetimes and makes shared mutable state (like open tabs or simulation buffers) explicit via Rc<RefCell<...>>.
- GHDL integration requires understanding how files are analyzed, elaborated, and simulated, and how .vcd files are generated.
- GtkSourceView 5 requires custom building in some distros, but its power is worth the effort.
- Supporting testbench detection (via std.env.stop, assert, etc.) simplifies user interaction.
- Auto-launching GTKWave from Rust is seamless with std::process::Command.

---

## 🔮 Future Features
- GHDL formatting with ghdl fmt
- Error annotation and navigation in the editor
- Project workspace config (via XML)
- Vivado and ModelSim integration
- RTL viewer or waveform embed via GTK3 interop
- Code completion (via LSP backend)
