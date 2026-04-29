use eframe::egui;

use crate::{cpu::StatusFlags, system::System};

pub struct DebuggerApp {
    pub system: System,
    rom_label: String,
}

impl DebuggerApp {
    pub fn new(system: System, rom_label: impl Into<String>) -> Self {
        Self {
            system,
            rom_label: rom_label.into(),
        }
    }

    fn hex(value: u32, digits: usize) -> String {
        format!("{:0width$X}", value, width = digits)
    }

    fn draw_ram(ui: &mut egui::Ui, bus: &crate::bus::Bus, start: u16, rows: usize, columns: usize, title: &str) {
        ui.monospace(title);
        let mut addr = start;
        for _ in 0..rows {
            let mut line = format!("${}:", Self::hex(addr as u32, 4));
            for _ in 0..columns {
                line.push_str(&format!(" {}", Self::hex(bus.read(addr, true) as u32, 2)));
                addr = addr.wrapping_add(1);
            }
            ui.monospace(line);
        }
    }

    fn draw_cpu(ui: &mut egui::Ui, cpu: &crate::cpu::Cpu) {
        ui.heading("CPU");
        ui.monospace(format!("PC: ${}", Self::hex(cpu.pc as u32, 4)));
        ui.monospace(format!("A:  ${}  [{}]", Self::hex(cpu.a as u32, 2), cpu.a));
        ui.monospace(format!("X:  ${}  [{}]", Self::hex(cpu.x as u32, 2), cpu.x));
        ui.monospace(format!("Y:  ${}  [{}]", Self::hex(cpu.y as u32, 2), cpu.y));
        ui.monospace(format!("SP: ${}", Self::hex(cpu.sp as u32, 2)));
        ui.monospace(format!("CLK: {}", cpu.complete()));

        ui.horizontal(|ui| {
            let flags = [
                (StatusFlags::N, "N"),
                (StatusFlags::V, "V"),
                (StatusFlags::U, "U"),
                (StatusFlags::B, "B"),
                (StatusFlags::D, "D"),
                (StatusFlags::I, "I"),
                (StatusFlags::Z, "Z"),
                (StatusFlags::C, "C"),
            ];
            for (flag, label) in flags {
                let color = if cpu.status.contains(flag) {
                    egui::Color32::LIGHT_GREEN
                } else {
                    egui::Color32::LIGHT_RED
                };
                ui.colored_label(color, label);
            }
        });
    }

    fn draw_code(ui: &mut egui::Ui, system: &System, lines: usize) {
        ui.heading("Disassembly");
        let pc = system.cpu.pc;
        let mut around: Vec<(u16, String)> = system
            .disassembly
            .range(..pc)
            .rev()
            .take(lines / 2)
            .map(|(&addr, text)| (addr, text.clone()))
            .collect();
        around.reverse();

        if let Some(current) = system.disassembly.get(&pc) {
            around.push((pc, current.clone()));
        }

        if pc < u16::MAX {
            for (&addr, text) in system.disassembly.range((pc + 1)..).take(lines / 2) {
                around.push((addr, text.clone()));
            }
        }

        for (addr, text) in around {
            if addr == pc {
                ui.label(
                    egui::RichText::new(text)
                        .monospace()
                        .color(egui::Color32::LIGHT_BLUE),
                );
            } else {
                ui.monospace(text);
            }
        }
    }
}

impl eframe::App for DebuggerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(0, 0, 170);
        visuals.window_fill = egui::Color32::from_rgb(0, 0, 170);
        visuals.override_text_color = Some(egui::Color32::WHITE);
        ctx.set_visuals(visuals);

        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.system.step_instruction();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::R)) {
            self.system.reset();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::I)) {
            self.system.irq();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::N)) {
            self.system.nmi();
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.heading("olc6502 Demonstration");
            ui.label(format!("Source: {}", self.rom_label));
            ui.label("SPACE = Step Instruction    R = RESET    I = IRQ    N = NMI");
        });

        egui::SidePanel::left("left")
            .resizable(true)
            .default_width(440.0)
            .show(ctx, |ui| {
            ui.heading("Memory");
            egui::ScrollArea::vertical().show(ui, |ui| {
                Self::draw_ram(ui, &self.system.bus, 0x0000, 16, 16, "Page 0000");
                ui.separator();
                Self::draw_ram(ui, &self.system.bus, 0x8000, 16, 16, "Page 8000");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(0, 0, 170))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            Self::draw_cpu(ui, &self.system.cpu);
                            ui.separator();
                            ui.monospace(format!("Clock count: {}", self.system.cpu.clock_count));
                            ui.separator();
                            Self::draw_code(ui, &self.system, 30);
                        });
                    });
            });
        });

        ctx.request_repaint();
    }
}
