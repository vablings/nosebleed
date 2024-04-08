#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Label, Sense, Ui};
use env_logger::Env;
use tasklist::Tasklist;

use crate::injector::allocator::VirtualAllocEx;
mod injector;

struct Injector {
    memory_writer: injector::writer::MemoryWriterMethod,
    executor: injector::executor::ExecutorMethod,
    allocator: injector::allocator::AllocatorMethod,
    pid: u32,
    search: String,
    picked_path: Option<String>,
    processes: Option<Tasklist>,
}
impl Default for Injector {
    fn default() -> Self {
        use injector::executor::CreateRemoteThread;
        use injector::writer::LoadLibary;
        Self {
            memory_writer: injector::writer::MemoryWriterMethod::LoadLibary(LoadLibary),
            executor: injector::executor::ExecutorMethod::CreateRemoteThread(CreateRemoteThread),
            allocator: injector::allocator::AllocatorMethod::VirtualAllocEx(VirtualAllocEx),
            pid: 0,
            search: String::new(),
            picked_path: None,
            processes: None,
        }
    }
}

impl eframe::App for Injector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.processes.is_none() {
            self.processes = unsafe { Some(tasklist::Tasklist::new()) };
        }

        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Open dll").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("DLL Files (.dll)", &["dll"])
                        .add_filter("All files", &["*"])
                        .pick_file()
                    {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
                ui.monospace(format!("PID: {} |", self.pid));
                if let Some(picked_path) = &self.picked_path {
                    ui.horizontal(|ui| {
                        ui.monospace(picked_path.split("\\").last().unwrap());
                    });
                }
            });
        });

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(90.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.search);
                });
                Ui::add_space(ui, 5.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for proc in self.processes.take().unwrap() {
                        if proc
                            .get_pname()
                            .to_ascii_lowercase()
                            .contains(&self.search.to_ascii_lowercase())
                        {
                            if ui
                                .add(
                                    Label::new(format!("{}", proc.get_pname()))
                                        .sense(Sense::click()),
                                )
                                .clicked()
                            {
                                self.pid = proc.get_pid();
                            }
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                use injector::writer::MemoryWriterMethod::*;
                ui.monospace("Module loading method:");
                ui.radio_value(
                    &mut self.memory_writer,
                    LoadLibary(injector::writer::LoadLibary),
                    "LoadLibary",
                );
                ui.radio_value(
                    &mut self.memory_writer,
                    ManualMap(injector::writer::ManualMap),
                    egui::RichText::new("ManualMap").strikethrough(),
                );

                use injector::executor::ExecutorMethod::*;
                ui.monospace("Execution method:");
                ui.radio_value(
                    &mut self.executor,
                    CreateRemoteThread(injector::executor::CreateRemoteThread),
                    "CreateRemoteThread",
                );
                ui.radio_value(
                    &mut self.executor,
                    ThreadHijacking(injector::executor::ThreadHijacking),
                    egui::RichText::new("Thread Hijacking").strikethrough(),
                );

                use injector::allocator::AllocatorMethod::*;
                ui.monospace("Allocator Method:");
                ui.radio_value(
                    &mut self.allocator,
                    VirtualAllocEx(injector::allocator::VirtualAllocEx),
                    "VirtualAllocEx",
                );
            });

            if ui.button("Inject").clicked() {
                let path = self.picked_path.clone().unwrap();

                injector::inject(
                    &self.allocator,
                    &self.memory_writer,
                    &self.executor,
                    self.pid,
                    path,
                );
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some([320.0, 250.0].into());

    eframe::run_native(
        "Nosebleed Injector",
        options,
        Box::new(|_cc| Box::new(Injector::default())),
    )
}
