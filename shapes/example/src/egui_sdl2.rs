use glam::{Quat, Vec3};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;

pub fn process_event(event: Event, raw_input: &mut egui::RawInput, state: &mut super::ExampleState) {
    match event {
        Event::Quit { .. } => state.should_quit = true,
        Event::MouseMotion { x, y, xrel, yrel, .. } => {
            raw_input.events.push(egui::Event::PointerMoved([x as f32, y as f32].into()));
            raw_input.events.push(egui::Event::MouseMoved([xrel as f32, yrel as f32].into()));
        },
        Event::MouseButtonDown { mouse_btn, clicks, x, y, .. } => {
            for _ in 0..clicks {
                raw_input.events.push(
                    egui::Event::PointerButton {
                        pos: [(x as u16).into(), (y as u16).into()].into(),
                        button: sdl2_mouse_button_to_egui_pointer_button(mouse_btn),
                        pressed: true,
                        modifiers: egui::Modifiers::NONE,
                    }
                );
            }
        },
        Event::MouseButtonUp { mouse_btn, clicks, x, y, .. } => {
            for _ in 0..clicks {
                raw_input.events.push(
                    egui::Event::PointerButton {
                        pos: [(x as u16).into(), (y as u16).into()].into(),
                        button: sdl2_mouse_button_to_egui_pointer_button(mouse_btn),
                        pressed: false,
                        modifiers: egui::Modifiers::NONE,
                    }
                );
            }
        },
        Event::TextInput { text, .. } => raw_input.events.push(egui::Event::Text(text)),
        Event::KeyUp { keycode: Some(keycode), keymod, repeat, .. } => {
            raw_input.events.push(
                egui::Event::Key {
                    key: sdl2_keycode_to_egui_key(keycode),
                    physical_key: None,
                    pressed: true,
                    repeat,
                    modifiers: sdl2_mod_to_egui_modifiers(keymod),
                }
            );
        },
        Event::KeyDown { keycode: Some(keycode), keymod, repeat, .. } => {
            raw_input.events.push(
                egui::Event::Key {
                    key: sdl2_keycode_to_egui_key(keycode),
                    physical_key: None,
                    pressed: false,
                    repeat,
                    modifiers: sdl2_mod_to_egui_modifiers(keymod),
                }
            );
            match keycode {
                Keycode::Up => match state.shape {
                    1 | 3 => state.shape = 2,
                    2 | 5 => state.shape = 4,
                    0 => {
                        state.sides = (state.sides + 1).min(255);
                        state.reset_polygon = true;
                    },
                    _ => (),
                },
                Keycode::Down => match state.shape {
                    4 => state.shape = 2,
                    2 => state.shape = 1,
                    0 => {
                        if state.sides > 3 {
                            state.sides -= 1;
                        }
                        state.reset_polygon = true;
                    },
                    _ => (),
                },
                Keycode::Left => match state.shape {
                    9 => state.shape = 8,
                    8 => state.shape = 7,
                    7 => state.shape = 6,
                    5 => state.shape = 3,
                    3 => state.shape = 1,
                    _ => (),
                },
                Keycode::Right => match state.shape {
                    1 | 2 => state.shape = 3,
                    3 | 4 => state.shape = 5,
                    6 => state.shape = 7,
                    7 => state.shape = 8,
                    8 => state.shape = 9,
                    _ => (),
                },
                Keycode::G => match state.shape {
                    0 => {
                        state.sides = 3;
                        state.reset_polygon = true;
                        state.shape = 5;
                    },
                    1 ..= 5 => state.shape = 6,
                    _ => state.shape = 0,
                },
                Keycode::R => state.rotating = !state.rotating,
                Keycode::KP_0 => state.rotation_delta = Quat::from_axis_angle(Vec3::ZERO, 0.01),
                Keycode::KP_1 => state.rotation_delta = Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.01),
                Keycode::KP_2 => state.rotation_delta = Quat::from_axis_angle(Vec3::new(0., 1., 0.), 0.01),
                Keycode::KP_3 => state.rotation_delta = Quat::from_axis_angle(Vec3::new(0., 1., 1.), 0.01),
                Keycode::KP_4 => state.rotation_delta = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.01),
                Keycode::KP_5 => state.rotation_delta = Quat::from_axis_angle(Vec3::new(1., 0., 1.), 0.01),
                Keycode::KP_6 => state.rotation_delta = Quat::from_axis_angle(Vec3::new(1., 1., 0.), 0.01),
                Keycode::KP_7 => state.rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01),
                Keycode::Minus => state.rotation_delta = state.rotation_delta.conjugate(),
                Keycode::H => state.rotation = state.initial_rotation,
                _ => (),
            }
        },
        _ => (), //eprintln!("{:?}", event),
    }
}

pub fn run_ui(ctx: &egui::Context, raw_input: egui::RawInput, state: &mut super::ExampleState) -> egui::FullOutput {
    const SPACER: f32 = 10.;
    ctx.run_ui(raw_input, |ui| {
        egui::Panel::left("left menu")
            .min_size(super::LEFT_PANEL.into())
            .max_size(super::LEFT_PANEL.into())
            .show(ui, |ui| {
                ui.separator();
                ui.heading("Global Menu");
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.add_space(SPACER);
                    if ui.add(egui::Button::new("Quit").shortcut_text("Alt+F4")).clicked() {
                        state.should_quit = true;
                    }
                    ui.add_space(SPACER);
                    let mut home_layout = Default::default();
                    egui::RichText::new("H").underline().append_to(
                        &mut home_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    egui::RichText::new("ome Position").append_to(
                        &mut home_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    if ui.button(home_layout).clicked() {
                        state.rotation = state.initial_rotation;
                    }
                    ui.add_space(SPACER);
                    let mut rotation_layout = Default::default();
                    egui::RichText::new("Toggle ").append_to(
                        &mut rotation_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    egui::RichText::new("R").underline().append_to(
                        &mut rotation_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    egui::RichText::new("otation").append_to(
                        &mut rotation_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    if ui.button(rotation_layout).clicked() {
                        state.rotating = !state.rotating;
                    }
                    ui.add_space(SPACER);
                    let mut reverse_layout = Default::default();
                    egui::RichText::new("Reverse Rotation ( ").append_to(
                            &mut reverse_layout,
                            &ctx.global_style(),
                            Default::default(),
                            Default::default(),
                            );
                    egui::RichText::new("-").underline().append_to(
                        &mut reverse_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    egui::RichText::new(" )").append_to(
                        &mut reverse_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    if ui.button(reverse_layout).clicked() {
                        state.rotation_delta = state.rotation_delta.conjugate();
                    }
                    ui.add_space(SPACER);
                    ui.label("Rotation Axis");
                    ui.horizontal(|ui| {
                        ui.add_space(2.25 * SPACER);
                        ui.group(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.horizontal(|ui| {
                                    if ui.button(" _ ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::ZERO, 0.01);
                                    }
                                    if ui.button(" Z ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.01);
                                    }
                                    if ui.button(" Y ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::new(0., 1., 0.), 0.01);
                                    }
                                    if ui.button("YZ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::new(0., 1., 1.), 0.01);
                                    }
                                });
                                ui.horizontal(|ui| {
                                    if ui.button(" X ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.01);
                                    }
                                    if ui.button("XZ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::new(1., 0., 1.), 0.01);
                                    }
                                    if ui.button("XY").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::new(1., 1., 0.), 0.01);
                                    }
                                    if ui.button("XYZ").clicked() {
                                        state.rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01);
                                    }
                                });
                            });
                        });
                    });
                    ui.add_space(SPACER);
                });
                ui.separator();
                ui.heading("Group Menu");
                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.add_space(SPACER);
                    let mut group_layout = Default::default();
                    egui::RichText::new("Cycle ").append_to(
                        &mut group_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    egui::RichText::new("G").underline().append_to(
                        &mut group_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    egui::RichText::new("roup").append_to(
                        &mut group_layout,
                        &ctx.global_style(),
                        Default::default(),
                        Default::default(),
                        );
                    if ui.button(group_layout).clicked() {
                        match state.shape {
                            0 => {
                                state.sides = 3;
                                state.reset_polygon = true;
                                state.shape = 5;
                            },
                            1 ..= 5 => state.shape = 6,
                            _ => state.shape = 0,
                        }
                    }
                    ui.add_space(SPACER);
                    match state.shape {
                        0 => {
                            ui.label("Polygon Sides");
                            ui.horizontal(|ui| {
                                ui.add_space(2. * SPACER);
                                let _ = ui.add(egui::Slider::new(&mut state.sides, 3..=u8::MAX as u16));
                                state.reset_polygon = true;
                            });
                        },
                        1..=5 => {
                            ui.label("Platonic Solid");
                            ui.horizontal(|ui| {
                                ui.add_space(4. * SPACER);
                                egui::ComboBox::from_id_salt(0)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut state.shape, 1, "Tetrahedron");
                                        ui.selectable_value(&mut state.shape, 2, "Hexahedron");
                                        ui.selectable_value(&mut state.shape, 3, "Octahedron");
                                        ui.selectable_value(&mut state.shape, 4, "Dodecahedron");
                                        ui.selectable_value(&mut state.shape, 5, "Icosahedron");
                                    });
                            });
                        },
                        6..=9 => {
                            ui.label("Kepler-Poinsot Polyhedron");
                            ui.horizontal(|ui| {
                                ui.add_space(4. * SPACER);
                                egui::ComboBox::from_id_salt(0)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut state.shape, 6, "Stellated Dodecahedron");
                                        ui.selectable_value(&mut state.shape, 7, "Great Dodecahedron");
                                        ui.selectable_value(&mut state.shape, 8, "Great Stellated Dodecahedron");
                                        ui.selectable_value(&mut state.shape, 9, "Great Icosahedron");
                                    });
                            });
                        },
                        _ => {
                            ui.label("Unknown group!");
                        },
                    }
                });
            });
        egui::Panel::top("top menu")
            .min_size(super::TOP_PANEL.into())
            .max_size(super::TOP_PANEL.into())
            .show(ui, |ui| {
                ui.add_space(SPACER);
                ui.label(
                    "Up and Down arrows modify vertices per face.\n\
                        Left and Right arrows modify faces per vertex.\n\
                        G switches between polyhedra and polygons.\n\
                        R toggles rotation.\n\
                        H returns object to initial orientation."
                        );
            });
    })
}

pub fn sdl2_keycode_to_egui_key(keycode: Keycode) -> egui::Key {
    match keycode {
        Keycode::ESCAPE => egui::Key::Escape,
        Keycode::KP_TAB | Keycode::TAB => egui::Key::Tab,
        Keycode::KP_SPACE | Keycode::SPACE => egui::Key::Space,
        Keycode::KP_ENTER | Keycode::RETURN | Keycode::RETURN2 => egui::Key::Enter,
        Keycode::BACKSPACE => egui::Key::Backspace,
        _ => egui::Key::IntlBackslash,
    }
}

pub fn sdl2_mouse_button_to_egui_pointer_button(mouse_button: MouseButton) -> egui::PointerButton {
    match mouse_button {
        MouseButton::Left => egui::PointerButton::Primary,
        MouseButton::Middle => egui::PointerButton::Middle,
        MouseButton::Right => egui::PointerButton::Secondary,
        _ => egui::PointerButton::Extra2,
    }
}

pub fn sdl2_mod_to_egui_modifiers(mods: Mod) -> egui::Modifiers {
    let mut modifiers = egui::Modifiers::default();
    if mods.contains(Mod::LSHIFTMOD) || mods.contains(Mod::RSHIFTMOD) {
        modifiers.shift = true;
    }
    modifiers
}
