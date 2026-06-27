use chip_eight::{Instruction, QuirksFields, QuirksMode, SuperChipBehaviour};
use iced::Task;

use crate::{
    ApplicationState, EmulatorKeyEvent, EmulatorKeyboardInputKind, Message, SupportedQuirksModes,
    controls_update::scroll_to_pc,
};

static CHARACTER_MAP: [char; 16] = [
    'x', '1', '2', '3', 'q', 'w', 'e', 'a', 's', 'd', 'z', 'c', '4', 'r', 'f', 'v',
];

pub fn next_instruction(application_state: &mut ApplicationState) -> Task<Message> {
    let emulator_state = application_state.emulator.0.borrow_mut().next();
    if let Some(emulator_state) = emulator_state {
        application_state.emulator_state = emulator_state;
    }
    let instruction = application_state.get_instruction_under_pc();
    let (x, y, h) = if let Some(instruction) = instruction {
        match instruction {
            Instruction::AddToRegister { register: x, .. }
            | Instruction::JumpWithOffset { register_x: x, .. }
            | Instruction::Random { register_x: x, .. }
            | Instruction::SetGeneralRegister { register: x, .. }
            | Instruction::SkipEqValueWithRegisterContents { register: x, .. }
            | Instruction::SkipIfKey { register: x, .. }
            | Instruction::SkipNotEqValueWithRegisterContents { register: x, .. }
            | Instruction::SubCommand { register: x, .. } => (Some(x), None, None),
            Instruction::SkipEqRegisters {
                register_x: x,
                register_y: y,
            }
            | Instruction::SkipNotEqRegisters {
                register_x: x,
                register_y: y,
            }
            | Instruction::LogicalOperator {
                register_x: x,
                register_y: y,
                ..
            } => (Some(x), Some(y), None),
            Instruction::Draw {
                x_register: x,
                y_register: y,
                height: h,
            } => (Some(x), Some(y), Some(h)),
            _ => (None, None, None),
        }
    } else {
        (None, None, None)
    };
    application_state.metadata.register_x = x;
    application_state.metadata.register_y = y;
    if let Some(height) = h {
        if height == 0 {
            application_state.metadata.draw_height = 16;
        } else {
            application_state.metadata.draw_height = height;
        }
    }

    scroll_to_pc(application_state)
}

pub fn update_quirks_mode(
    application_state: &mut ApplicationState,
    new_mode: SupportedQuirksModes,
) -> Task<Message> {
    let quirks_mode = match &new_mode {
        SupportedQuirksModes::Chip8 => QuirksMode::Chip8,
        SupportedQuirksModes::SuperChip => QuirksMode::SuperChip(SuperChipBehaviour::Modern),
    };
    application_state.quirks_mode = new_mode;

    let quirks = chip_eight::QuirksFields::from(quirks_mode);
    application_state
        .emulator
        .0
        .borrow_mut()
        .with_custom_quirks(QuirksFields {
            disp_wait: false,
            ..quirks
        });
    Task::none()
}

pub fn toggle_running(application_state: &mut ApplicationState) -> Task<Message> {
    application_state.is_running = !application_state.is_running;
    Task::none()
}

pub fn set_execution_speed(
    application_state: &mut ApplicationState,
    new_speed: u8,
) -> Task<Message> {
    application_state.execution_speed = new_speed;
    Task::none()
}

pub fn respond_to_key_event(
    application_state: &mut ApplicationState,
    e: EmulatorKeyEvent,
    c: EmulatorKeyboardInputKind,
) -> Task<Message> {
    let position = match c {
        EmulatorKeyboardInputKind::UsKeyboardChar(c) => {
            let Some(position) = CHARACTER_MAP
                .iter()
                .position(|inner| *inner == c.to_ascii_lowercase())
            else {
                return Task::none();
            };
            (position & 0xF) as u8
        }
        EmulatorKeyboardInputKind::HexKeyIndex(position) => position,
    };

    if let Some(key) = application_state
        .emulator
        .0
        .borrow_mut()
        .input_provider
        .keys_state
        .get_mut(position as usize & 0xF)
    {
        match e {
            EmulatorKeyEvent::Up => *key = 0,
            EmulatorKeyEvent::Down => *key = 1,
            EmulatorKeyEvent::Toggle => *key = if *key == 0 { 1 } else { 0 },
        };
    };
    Task::none()
}

pub fn update_program(application_state: &mut ApplicationState, program: Vec<u8>) -> Task<Message> {
    application_state.fetching_data = false;
    application_state.current_program = program
        .chunks(2)
        .map(|c| {
            if let (Some(a), Some(b)) = (c.first(), c.get(1)) {
                ((*a as u16) << 8 | *b as u16).into()
            } else {
                Instruction::Unimplemented(0)
            }
        })
        .collect();
    if let Err(e) = application_state.emulator.0.borrow_mut().reset(program) {
        eprintln!("Program too large: {e}");
    };

    let quirks = chip_eight::QuirksFields::from(chip_eight::QuirksMode::Chip8);
    application_state
        .emulator
        .0
        .borrow_mut()
        .with_custom_quirks(QuirksFields {
            disp_wait: false,
            ..quirks
        });

    scroll_to_pc(application_state)
}
