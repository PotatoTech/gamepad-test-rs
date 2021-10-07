// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2021 PotatoTech

use sdl2::controller::{Axis, Button, GameController};
use sdl2::GameControllerSubsystem;
use std::collections::HashMap;
use std::path::Path;

pub fn handle_gamepad_added(
    contoller_subsystem: &GameControllerSubsystem,
    gamepads: &mut HashMap<u32, GameController>,
    joystick_index: u32,
    show_mappings: bool,
) {
    match contoller_subsystem.open(joystick_index) {
        Ok(gamepad) => {
            let id = gamepad.instance_id();
            gamepads.insert(id, gamepad);
            println!("Added gamepad {}", id);
            if show_mappings {
                println!("mapping: {}", gamepads.get(&id).unwrap().mapping());
            }
        }
        Err(_) => eprintln!("Could not open device {}", joystick_index),
    }
}

pub fn handle_gamepad_removed(gamepads: &mut HashMap<u32, GameController>, joystick_id: u32) {
    gamepads.remove(&joystick_id);
    println!("Removed gamepad {}", joystick_id);
}

pub fn get_axis_name(axis: Axis) -> &'static str {
    match axis {
        Axis::LeftX => "LeftX",
        Axis::LeftY => "LeftY",
        Axis::RightX => "RightX",
        Axis::RightY => "RightY",
        Axis::TriggerLeft => "TriggerLeft",
        Axis::TriggerRight => "TriggerRight",
    }
}

pub fn get_button_name(button: Button) -> &'static str {
    match button {
        Button::A => "A",
        Button::B => "B",
        Button::X => "X",
        Button::Y => "Y",
        Button::Back => "Back",
        Button::Guide => "Guide",
        Button::Start => "Start",
        Button::LeftStick => "LeftStick",
        Button::RightStick => "RightStick",
        Button::LeftShoulder => "LeftShoulder",
        Button::RightShoulder => "RightShoulder",
        Button::DPadUp => "DPadUp",
        Button::DPadDown => "DPadDown",
        Button::DPadLeft => "DPadLeft",
        Button::DPadRight => "DPadRight",
    }
}

pub fn handle_axis_motion(joystick_id: u32, axis: Axis, value: i16) {
    println!(
        "id {}: axis {} = {}",
        joystick_id,
        get_axis_name(axis),
        value
    );
}

pub fn handle_button_down(joystick_id: u32, button: Button) {
    println!(
        "id {}: button {} = down",
        joystick_id,
        get_button_name(button)
    );
}

pub fn handle_button_up(joystick_id: u32, button: Button) {
    println!(
        "id {}: button {} = up",
        joystick_id,
        get_button_name(button)
    );
}

pub fn load_mappings<P: AsRef<Path>>(contoller_subsystem: &GameControllerSubsystem, path: P) {
    match contoller_subsystem.load_mappings(path) {
        Ok(_) => (),
        Err(_) => eprintln!("Could not load mappings file"),
    }
}
