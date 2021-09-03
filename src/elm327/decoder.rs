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

#[derive(Debug, Clone)]
pub enum AirStatus {
    Upstream,
    Downstream,
    FromTheOutsideAtmosphereOrOff,
    PumpCommandedOnForDiagnostics,
    Unknow
}


#[derive(Debug, Clone)]
pub enum ObdStandard {
    Obd2CARB,   ObdEPA, Obd1and2,   Obd1,   NotObdCompliant,    Eobd,   EobdAndObd2,    EobdAndObd, EobdAndObd2AndObd,  Jobd,
    JobdAndObd2,    JobdAndEobd,    JobdAndEobdAndObd2, Emd,    EmdPlus,    HdObdC, HdObd,  WwhObd, HdEobd1,    HdEobd1N,   
    HdEobd2,    HdEobd2N,   ObdBr1, ObdBr2, Kobd,   Iobd1,  Iobd2,  HdEobd6,    NotAvailableForAssignement, Reserved, Unknow, Value(u8)
}

#[derive(Debug, Clone)]
pub enum State {
    On, Off, Unknow
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

pub fn decode_timing_advance(input: u8) -> f64 {
    input as f64 / 2.0 - 64.0
}

pub fn decode_rpm(input: u16) -> f64 {
    ( ((input>>8)*256) + (input&0xff) ) as f64 / 4.0
}

pub fn decode_maf(input: u16) -> f64 {
    ( ((input>>8)*256) + (input&0xff) ) as f64 / 4.0
}

pub fn decode_percent(input: u8) -> f64 {
    input as f64 / 2.55
}

pub fn decode_fuel_trim(input: u8) -> f64 {
    input as f64 / 1.28 - 100.0
}

pub fn decode_air_status(input: u8) -> AirStatus {
    match input {
        0x01 => AirStatus::Upstream,
        0x02 => AirStatus::Downstream,
        0x04 => AirStatus::FromTheOutsideAtmosphereOrOff,
        0x08 => AirStatus::PumpCommandedOnForDiagnostics,
        _    => AirStatus::Unknow
    }
}

pub fn decode_oxygen_sensor(input: u16) -> (f64, f64) {
    let a : f64 = (input >> 8) as f64;
    let b : f64 = (input & 0xff) as f64;
    (
        a / 200.0,
        { if b != 255.0 {100.0 * b / 128.0 - 100.0} else {0.0} }
    )
}

pub fn decode_obd_standard(input: u8) -> ObdStandard {
    match input {
        1  => ObdStandard::Obd2CARB,
        2  => ObdStandard::ObdEPA,
        3  => ObdStandard::Obd1and2,
        4  => ObdStandard::Obd1,
        5  => ObdStandard::NotObdCompliant,
        6  => ObdStandard::Eobd,
        7  => ObdStandard::EobdAndObd2,
        8  => ObdStandard::EobdAndObd,
        9  => ObdStandard::EobdAndObd2AndObd,
        10 => ObdStandard::Jobd,
        11 => ObdStandard::JobdAndObd2,
        12 => ObdStandard::JobdAndEobd,
        13 => ObdStandard::JobdAndEobdAndObd2,
        17 => ObdStandard::Emd,
        18 => ObdStandard::EmdPlus,
        19 => ObdStandard::HdObdC,
        20 => ObdStandard::HdObd,
        21 => ObdStandard::WwhObd,
        23 => ObdStandard::HdEobd1,
        24 => ObdStandard::HdEobd1N,
        25 => ObdStandard::HdEobd2,
        26 => ObdStandard::HdEobd2N,
        28 => ObdStandard::ObdBr1,
        29 => ObdStandard::ObdBr2,
        30 => ObdStandard::Kobd,
        31 => ObdStandard::Iobd1,
        32 => ObdStandard::Iobd2,
        33 => ObdStandard::HdEobd6,
        14 ..= 16 | 22 | 27 | 34 ..= 250 => ObdStandard::Reserved,
        251 ..= 255 => ObdStandard::NotAvailableForAssignement,
        0 => ObdStandard::Unknow
    }
}

pub fn decode_auxiliary_input_status(input: u8) -> State {
    match input >> 7 {
        0 => State::Off,
        1 => State::On,
        _ => State::Unknow
    }
}

pub fn decode_seconds(input: u16) -> u16 {
    256*(input>>8) + (input&0xff)
}

pub fn decode_km(input: u16) -> u16 {
    256*(input>>8) + (input&0xff)
}

pub fn decode_fuel_rail_pressure(input: u16) -> f64 {
    0.079 * (256.0*(input>>8) as f64 + (input&0xff) as f64)
}

pub fn decode_fuel_rail_gauge_pressure(input: u16) -> u32 {
    10 * (256*(input>>8) as u32 + (input&0xff) as u32)
}

pub fn decode_oxygen_sensor_lambda(input: u32) -> (f64, f64) {
    let a : f64 = (input >> 24) as f64;
    let b : f64 = ((input >> 16) & 0xff) as f64;
    let c : f64 = ((input >> 8) & 0xff) as f64;
    let d : f64 = (input & 0xff) as f64;
    (
        (2.0/65536.0) * (256.0*a as f64 + b),
        (8.0/65536.0) * (256.0*c as f64 + d)
    )
}

pub fn decode_egr_error(input: u8) -> f64 {
    input as f64 / 1.28 - 100.0
}