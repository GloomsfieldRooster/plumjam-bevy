use std::collections::HashMap;

use bevy::{
    input::{gamepad::{GamepadAxisChangedEvent, GamepadButtonInput}, keyboard::KeyboardInput, mouse::{MouseButtonInput, MouseMotion}}, prelude::*
};

#[derive(PartialEq, Eq, Hash)]
pub enum InputDirection {
    PositiveHorizontal,
    NegativeHorizontal,
    PositiveVertical,
    NegativeVertical
}

#[derive(PartialEq, Eq, Hash)]
pub enum InputType {
    KeyboardKey { key_code: KeyCode },
    MouseButton { button_code: MouseButton },
    MouseMovement { direction: InputDirection },
    GamepadButton { button_code: GamepadButton },
    GamepadAxis { axis: GamepadAxisType, axis_direction: InputDirection },
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum InputMappables {
    DirectionUp,
    DirectionDown,
    DirectionLeft,
    DirectionRight,

    LookDeltaLeft,
    LookDeltaRight,
    LookDeltaUp,
    LookDeltaDown,
}

#[derive(Resource)]
pub struct Inputs {
    pub direction_up: f32,
    pub direction_down: f32,
    pub direction_left: f32,
    pub direction_right: f32,

    pub look_delta_left: f32,
    pub look_delta_right: f32,
    pub look_delta_up: f32,
    pub look_delta_down: f32,

    pub input_maps: HashMap<InputType, InputMappables>,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            direction_up: 0.0,
            direction_down: 0.0,
            direction_left: 0.0,
            direction_right: 0.0,
            look_delta_left: 0.0,
            look_delta_down: 0.0,
            look_delta_right: 0.0,
            look_delta_up: 0.0,
            input_maps: HashMap::from([
                (InputType::MouseMovement { direction: InputDirection::PositiveVertical }, InputMappables::DirectionUp),
                (InputType::KeyboardKey { key_code: KeyCode::KeyS }, InputMappables::DirectionDown),
                (InputType::KeyboardKey { key_code: KeyCode::KeyA }, InputMappables::DirectionLeft),
                (InputType::KeyboardKey { key_code: KeyCode::KeyD }, InputMappables::DirectionRight),
                (InputType::MouseMovement { direction: InputDirection::NegativeHorizontal }, InputMappables::LookDeltaLeft),
                (InputType::MouseMovement { direction: InputDirection::PositiveHorizontal }, InputMappables::LookDeltaRight),
                (InputType::MouseMovement { direction: InputDirection::NegativeVertical }, InputMappables::LookDeltaDown),
                // (InputType::MouseMovement { direction: InputDirection::PositiveVertical }, InputMappables::LookDeltaUp),
            ])
        }
    }
}

fn parse_input(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_input_events: EventReader<MouseMotion>,
    mut gamepad_button_input_events: EventReader<GamepadButtonInput>,
    mut gamepad_axis_input_events: EventReader<GamepadAxisChangedEvent>,

    mut inputs: ResMut<Inputs>,
) {
    let mut events: Vec<(InputType, f32)> = Vec::new();

    for keyboard_event in keyboard_input_events.read() {
        events.push((InputType::KeyboardKey {
            key_code: keyboard_event.key_code 
        }, 
        keyboard_event.state.is_pressed() as u8 as f32));
    }

    for mouse_button_event in mouse_button_input_events.read() {
        events.push((InputType::MouseButton {
            button_code: mouse_button_event.button
        },
        mouse_button_event.state.is_pressed() as u8 as f32));
    }

    for mouse_motion_event in mouse_motion_input_events.read() {
        let delta_x: f32 = mouse_motion_event.delta.x;
        let delta_y: f32 = mouse_motion_event.delta.y;

        let x_direction: InputDirection;
        let y_direction: InputDirection;

        if delta_x >= 0.0 { x_direction = InputDirection::PositiveHorizontal; }
        else { x_direction = InputDirection::NegativeHorizontal; }

        if delta_y <= 0.0 { y_direction = InputDirection::PositiveVertical; }
        else { y_direction = InputDirection::NegativeVertical; }

        events.push((InputType::MouseMovement { direction: x_direction }, delta_x));
        events.push((InputType::MouseMovement { direction: y_direction }, delta_y));
    }

    

    for gamepad_button_event in gamepad_button_input_events.read() {
        events.push((InputType::GamepadButton {
            button_code: gamepad_button_event.button
        },
        gamepad_button_event.state.is_pressed() as u8 as f32));
    }

    for gamepad_axis_event in gamepad_axis_input_events.read() {
        let direction: InputDirection;
        let value: f32 = gamepad_axis_event.value;

        match gamepad_axis_event.axis_type {
            GamepadAxisType::LeftStickX => {
                if value >= 0.0 {
                    direction = InputDirection::PositiveHorizontal;
                }
                else {
                    direction = InputDirection::NegativeHorizontal;
                }
            }
            GamepadAxisType::LeftStickY => {
                if value >= 0.0 {
                    direction = InputDirection::PositiveVertical;
                }
                else {
                    direction = InputDirection::NegativeVertical
                }
            }
            GamepadAxisType::RightStickX => {
                if value >= 0.0 {
                    direction = InputDirection::PositiveHorizontal;
                }
                else {
                    direction = InputDirection::NegativeHorizontal;
                }
            }
            GamepadAxisType::RightStickY => {
                if value >= 0.0 {
                    direction = InputDirection::PositiveVertical;
                }
                else {
                    direction = InputDirection::NegativeVertical;
                }
            }
            _ => { direction = InputDirection::NegativeHorizontal; }
        }
        
        events.push((InputType::GamepadAxis {
            axis: gamepad_axis_event.axis_type,
            axis_direction: direction
        },
        gamepad_axis_event.value as f32));
    }

    for event in events.iter() {
        match inputs.input_maps.get_key_value(&event.0) {
            Some(value) => {
                match value.1 {
                    InputMappables::DirectionUp => { inputs.direction_up = event.1; },
                    InputMappables::DirectionDown => { inputs.direction_down = event.1; },
                    InputMappables::DirectionLeft => { inputs.direction_left = event.1; },
                    InputMappables::DirectionRight => { inputs.direction_right = event.1; },
                    InputMappables::LookDeltaLeft => { inputs.look_delta_left = event.1; },
                    InputMappables::LookDeltaRight => { inputs.look_delta_right = event.1; },
                    InputMappables::LookDeltaUp => { inputs.look_delta_up = event.1; }
                    InputMappables::LookDeltaDown => { inputs.look_delta_down = event.1; }
                }
            }
            None => { }
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, parse_input);
    }
}