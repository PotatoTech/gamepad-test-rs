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

pub fn handle_axis_motion(joystick_id: u32, axis: Axis, value: i16, deadzone: u16) {
    if deadzone == 0 || deadzone < value.unsigned_abs() {
        println!("id {}: axis {:?} = {}", joystick_id, axis, value);
    }
}

pub fn handle_button_down(joystick_id: u32, button: Button) {
    println!("id {}: button {:?} = down", joystick_id, button);
}

pub fn handle_button_up(joystick_id: u32, button: Button) {
    println!("id {}: button {:?} = up", joystick_id, button);
}

pub fn load_mappings<P: AsRef<Path>>(contoller_subsystem: &GameControllerSubsystem, path: P) {
    match contoller_subsystem.load_mappings(path) {
        Ok(_) => (),
        Err(_) => eprintln!("Could not load mappings file"),
    }
}
