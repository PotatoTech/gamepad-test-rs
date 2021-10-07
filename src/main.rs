// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2021 PotatoTech

mod gamepad;

use clap::{AppSettings, Clap};
use gamepad::*;
use sdl2::controller::GameController;
use sdl2::event::Event;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let opts = Opts::parse();

    let sdl_context = sdl2::init().unwrap();
    let contoller_subsystem = sdl_context.game_controller().unwrap();

    if let Some(path) = opts.mappings {
        load_mappings(&contoller_subsystem, path);
    }

    let mut gamepads = HashMap::<u32, GameController>::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    handle_gamepad_added(&contoller_subsystem, &mut gamepads, which, opts.verbose)
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    handle_gamepad_removed(&mut gamepads, which)
                }
                Event::ControllerAxisMotion {
                    which, axis, value, ..
                } => handle_axis_motion(which, axis, value),
                Event::ControllerButtonDown { which, button, .. } => {
                    handle_button_down(which, button)
                }
                Event::ControllerButtonUp { which, button, .. } => handle_button_up(which, button),
                Event::Quit { .. } => break 'running,
                _ => (),
            }
        }
    }
}

#[derive(Clap)]
#[clap(setting = AppSettings::DisableVersionFlag)]
struct Opts {
    /// Load the mappings from the specified file
    #[clap(short, long)]
    mappings: Option<PathBuf>,
    /// Show the mapping used by each controller when it is added
    #[clap(short, long)]
    verbose: bool,
}
