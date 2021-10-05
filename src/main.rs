// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2021 PotatoTech

use sdl2::controller::{Axis, Button, GameController};
use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use std::collections::HashMap;

fn handle_gamepad_added(
    contoller_subsystem: &GameControllerSubsystem,
    gamepads: &mut HashMap<u32, GameController>,
    joystick_index: u32,
) {
    match contoller_subsystem.open(joystick_index) {
        Ok(gamepad) => {
            let id = gamepad.instance_id();
            gamepads.insert(id, gamepad);
            println!("Added gamepad {}", id);
        }
        Err(_) => eprintln!("Could not open device {}", joystick_index),
    }
}

fn handle_gamepad_removed(gamepads: &mut HashMap<u32, GameController>, joystick_id: u32) {
    gamepads.remove(&joystick_id);
    println!("Removed gamepad {}", joystick_id);
}

fn get_axis_name(axis: Axis) -> &'static str {
    match axis {
        Axis::LeftX => "LeftX",
        Axis::LeftY => "LeftY",
        Axis::RightX => "RightX",
        Axis::RightY => "RightY",
        Axis::TriggerLeft => "TriggerLeft",
        Axis::TriggerRight => "TriggerRight",
    }
}

fn get_button_name(button: Button) -> &'static str {
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

fn handle_axis_motion(joystick_id: u32, axis: Axis, value: i16) {
    println!(
        "id {}: axis {} = {}",
        joystick_id,
        get_axis_name(axis),
        value
    );
}

fn handle_button_down(joystick_id: u32, button: Button) {
    println!(
        "id {}: button {} = down",
        joystick_id,
        get_button_name(button)
    );
}

fn handle_button_up(joystick_id: u32, button: Button) {
    println!(
        "id {}: button {} = up",
        joystick_id,
        get_button_name(button)
    );
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let contoller_subsystem = sdl_context.game_controller().unwrap();

    let mut gamepads = HashMap::<u32, GameController>::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    handle_gamepad_added(&contoller_subsystem, &mut gamepads, which)
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
