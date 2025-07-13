mod ghdl_runner;

use ghdl_runner::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, FileChooserAction, FileChooserDialog, Orientation, PolicyType, ResponseType, ScrolledWindow};
use gtksourceview::prelude::*;
use gtksourceview::{Buffer, LanguageManager, View};
use std::{cell::RefCell, fs, path::PathBuf};
use std::rc::Rc;
use std::path::Path;

struct TabData {
    buffer: Buffer,
    file_path: Option<PathBuf>
}
type TabList = Rc<RefCell<Vec<TabData>>>;

fn main() {
    let app = Application::builder()
        .application_id("com.gabibdods.vhsichdl")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let notebook = gtk::Notebook::new();
    notebook.set_tab_pos(gtk::PositionType::Top);

    let output_buffer = gtk::TextBuffer::new(None);
    let output_view = gtk::TextView::with_buffer(&output_buffer);
    output_view.set_editable(false);
    output_view.set_monospace(true);
    output_view.set_vexpand(true);
    output_view.set_wrap_mode(gtk::WrapMode::WordChar);
    let output_scroll = ScrolledWindow::builder()
        .child(&output_view)
        .min_content_height(150)
        .hexpand(false)
        .vexpand(false)
        .build();
    output_scroll.set_policy(PolicyType::Automatic, PolicyType::Automatic);

    let entity_selector = gtk::ComboBoxText::new();
    entity_selector.set_hexpand(true);
    let entity_box = Box::new(Orientation::Horizontal, 6);
    let label = gtk::Label::new(Some("Top-Level Entity/Testbench:"));
    entity_box.append(&label);
    entity_box.append(&entity_selector);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("VhsicHdl")
        .default_width(800)
        .default_height(600)
        .build();
    let lang_manager = LanguageManager::new();

    let buffer = Buffer::new(None);
    if let Some(vhdl_lang) = lang_manager.language("vhdl") {
        buffer.set_language(Some(&vhdl_lang));
    }

    let view = View::with_buffer(&buffer);
    view.set_show_line_numbers(true);
    view.set_highlight_current_line(true);
    view.set_monospace(true);

    let scroll = ScrolledWindow::builder()
        .child(&view)
        .hexpand(true)
        .vexpand(true)
        .build();
    scroll.set_policy(PolicyType::Automatic, PolicyType::Automatic);

    let buffer_rc = Rc::new(buffer);
    let window_rc = Rc::new(window.clone());
    let file_rc = Rc::new(RefCell::new(None::<PathBuf>));
    let notebook_rc = Rc::new(notebook.clone());
    let tabs_rc: TabList = Rc::new(RefCell::new(Vec::new()));
    let output_buffer_rc = Rc::new(output_buffer);
    let entity_selector_rc = Rc::new(entity_selector);

    let open_action = gio::SimpleAction::new("open", None);
    {
        let window = window_rc.clone();
        let tabs = tabs_rc.clone();
        let notebook = notebook_rc.clone();
        let entity_selector = entity_selector_rc.clone();
        open_action.connect_activate(move |_, _| {
            let dialog = FileChooserDialog::new(
                Some("Open File"),
                Some(&*window),
                FileChooserAction::Open,
                &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
            );
            let notebook = notebook.clone();
            let tabs = tabs.clone();
            let entity_selector = entity_selector.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(path) = dialog.file().and_then(|f| f.path()) {
                        open_file_in_tab(&notebook, &path, &tabs, &entity_selector);
                    }
                }
                dialog.close();
            });
            dialog.show();
        });
    }
    app.add_action(&open_action);

    let save_action = gio::SimpleAction::new("save", None);
    {
        let notebook = notebook_rc.clone();
        let tabs = tabs_rc.clone();
        save_action.connect_activate(move |_, _| {
            let page = notebook.current_page();
            if page < Some(0) {
                return;
            }
            if let Some(page) = notebook.current_page() {
                if let Some(tab) = tabs.borrow_mut().get_mut(page as usize) {
                    if let Some(path) = &tab.file_path {
                        let text = tab.buffer.text(&tab.buffer.start_iter(), &tab.buffer.end_iter(), false);
                        if let Err(e) = fs::write(path, text.as_str()) {
                            eprintln!("Error saving file: {e}");
                        }
                    } else {
                        eprintln!("No file name set");
                    }
                }
            }
        });
    }
    app.add_action(&save_action);

    let save_as_action = gio::SimpleAction::new("save_as", None);
    {
        let buffer = buffer_rc.clone();
        let file = file_rc.clone();
        let window = window_rc.clone();
        save_as_action.connect_activate(move |_, _| {
            let dialog = FileChooserDialog::new(
                Some("Save File As"),
                Some(&*window),
                FileChooserAction::Save,
                &[("Cancel", ResponseType::Cancel), ("Save", ResponseType::Accept)],
            );
            let buffer = buffer.clone();
            let file = file.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(path) = dialog.file().and_then(|f| f.path()) {
                        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
                        if fs::write(&path, text.as_str()).is_ok() {
                            *file.borrow_mut() = Some(path.clone());
                        } else {
                            eprintln!("Failed to save file");
                        }
                    }
                }
                dialog. close();
            });
            dialog.show();
        });
    }
    app.add_action(&save_as_action);

    let autosave_action = gio::SimpleAction::new("autosave", None);
    {
        let tabs = tabs_rc.clone();
        autosave_action.connect_activate(move |_, _| {
            for tab in tabs.borrow().iter() {
                if let Some(path) = &tab.file_path {
                    let text = tab.buffer.text(&tab.buffer.start_iter(), &tab.buffer.end_iter(), false);
                    if let Err(e) = fs::write(path, text.as_str()) {
                        eprintln!("Autosave failed: {e}");
                    }
                }
            }
        });
    }
    app.add_action(&autosave_action);

    let simulate_action = gio::SimpleAction::new("simulate", None);
    {
        let tabs = tabs_rc.clone();
        let notebook = notebook_rc.clone();
        let output_buffer = output_buffer_rc.clone();
        simulate_action.connect_activate(move |_, _| {
            if let Some(page) = notebook.current_page() {
                if let Some(tab) = tabs.borrow().get(page as usize) {
                    if let Some(path) = &tab.file_path {
                        output_buffer.set_text("");
                        match ghdl_analyze(path) {
                            Ok(output) => {
                                let mut iter = output_buffer.end_iter();
                                output_buffer.insert(&mut iter, ">>> ghdl -a output:\n");
                                output_buffer.insert(&mut output_buffer.end_iter(), &String::from_utf8_lossy(&output.stdout));
                                output_buffer.insert(&mut output_buffer.end_iter(), &String::from_utf8_lossy(&output.stderr));
                            }
                            Err(e) => eprintln!("Analyze failed: {e}"),
                        }
                        let entity = path.file_stem().unwrap().to_str().unwrap();
                        match ghdl_elaborate(entity) {
                            Ok(output) => {
                                let mut iter = output_buffer.end_iter();
                                output_buffer.insert(&mut iter, ">>> ghdl -e output:\n");
                                output_buffer.insert(&mut output_buffer.end_iter(), &String::from_utf8_lossy(&output.stdout));
                                output_buffer.insert(&mut output_buffer.end_iter(), &String::from_utf8_lossy(&output.stderr));
                            }
                            Err(e) => eprintln!("Elaboration failed: {e}"),
                        }
                        match ghdl_run(entity, Some(Path::new("wave.vcd"))) {
                            Ok(output) => {
                                let mut iter = output_buffer.end_iter();
                                output_buffer.insert(&mut iter, ">>> ghdl -r output:\n");
                                output_buffer.insert(&mut output_buffer.end_iter(), &String::from_utf8_lossy(&output.stdout));
                                output_buffer.insert(&mut output_buffer.end_iter(), &String::from_utf8_lossy(&output.stderr));
                            }
                            Err(e) => eprintln!("Simulation run failed: {e}"),
                        }
                    }
                }
            }
        });
    }
    app.add_action(&simulate_action);

    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate(move |_, _| {
        let window = window_rc.clone();
        window.close();
    });
    app.add_action(&quit_action);

    let file_menu = gio::Menu::new();
    file_menu.append(Some("Open"), Some("app.open"));
    file_menu.append(Some("Save"), Some("app.save"));
    file_menu.append(Some("Save As"), Some("app.save_as"));
    file_menu.append(Some("Autosave"), Some("app.autosave"));
    file_menu.append(Some("Simulate"), Some("app.simulate"));
    file_menu.append(Some("Quit"), Some("app.quit"));
    let menubar_model = gio::Menu::new();
    menubar_model.append_submenu(Some("File"), &file_menu);
    let menubar = gtk::PopoverMenuBar::builder()
        .menu_model(&menubar_model)
        .build();

    let layout = Box::new(Orientation::Vertical, 0);
    layout.append(&menubar);
    layout.append(&notebook);
    layout.append(&output_scroll);
    layout.append(&entity_box);

    window.set_child(Some(&layout));
    window.present();
}

fn open_file_in_tab(notebook: &gtk::Notebook, path: &PathBuf, tabs: &TabList, entity_selector: &gtk::ComboBoxText) {
    let buffer = Buffer::new(None);
    if let Ok(content) = fs::read_to_string(path) {
        buffer.set_text(&content);
    }

    let view = View::with_buffer(&buffer);
    view.set_show_line_numbers(true);
    view.set_highlight_current_line(true);
    view.set_monospace(true);

    let scroll = ScrolledWindow::builder()
        .child(&view)
        .hexpand(true)
        .vexpand(true)
        .build();
    scroll.set_policy(PolicyType::Automatic, PolicyType::Automatic);

    if let Ok(content) = fs::read_to_string(path) {
        buffer.set_text(&content);
        let is_tb = is_testbench(&content);
        for entity in detect_entities(&content) {
            let label = if is_tb {
                format!("{entity} (testbench")
            } else {
                entity.clone()
            };
            entity_selector.append_text(&label);
        }
    }

    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Untitled");

    let label = gtk::Label::new(Some(file_name));
    notebook.append_page(&scroll, Some(&label));

    tabs.borrow_mut().push(TabData {
        buffer: buffer.clone(),
        file_path: Some(path.clone())
    });
}

fn detect_entities(text: &str) -> Vec<String> {
    let mut entities = Vec::new();
    for line in text.lines() {
        if let Some(name) = line.trim().strip_prefix("entity ") {
            if let Some(end) = name.find(" is ") {
                entities.push(name[..end].trim().to_string());
            }
        }
    }
    entities
}

fn is_testbench(text: &str) -> bool {
    text.contains("std.env.stop") || text.contains("assert") || text.contains("wait") || text.contains("report")
}