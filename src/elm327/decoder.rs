// Uses
use crate::elm327::types::*;

// Enums
#[derive(Debug, Clone)]
pub enum FuelSystem {
    MotorOff,
    OpenLoopInsufficientEngineTemperature,
    ClosedLoopUsingOxygenSensorFeedback,
    OpenLoopEngineLoadOrFuelCutDueToDeceleration,
    OpenLoopSystemFailure,
    ClosedLoopFaultFeedbackSystem,
    Unknow
}

// Fonctions
pub fn decode_celsius(encoded: u8) -> i16 {
    encoded as i16 - 40
}

pub fn decode_available_pids(input: u32, pid_offset: PidLen) -> Vec<PidLen> {
    let     base           : u32         = 2u32.pow(31);
    let mut available_pids : Vec<PidLen> = vec![];
    for offset in 0..=31 {
        if base>>offset & input != 0 { available_pids.push(offset+1+32*pid_offset); }
    }
    available_pids
}

fn interpret_one_byte_fuel(b: u8) -> FuelSystem {
    match b {
        0  => FuelSystem::MotorOff,
        1  => FuelSystem::OpenLoopInsufficientEngineTemperature,
        2  => FuelSystem::ClosedLoopUsingOxygenSensorFeedback,
        4  => FuelSystem::OpenLoopEngineLoadOrFuelCutDueToDeceleration,
        8  => FuelSystem::OpenLoopSystemFailure,
        16 => FuelSystem::ClosedLoopFaultFeedbackSystem,
        _  => FuelSystem::Unknow
    }
}

pub fn decode_fuel_system(input: u16) -> (FuelSystem, FuelSystem) {
    (
        interpret_one_byte_fuel((input & 0xff) as u8),
        interpret_one_byte_fuel((input >> 8) as u8)            
    )
}