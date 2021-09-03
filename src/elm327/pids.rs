// Uses
use crate::elm327::decoder::*;
use crate::elm327::types::*;

// Enums
#[derive(Debug, Clone)]
pub enum ResultSize {
    Range(usize, usize),
    Value(usize)
}

// Définission du format d'un PID
pub trait Pid {
    //
    // Trait définissant ce que doit implémenter un pid obd2
    // :type Input:            Le type de la donnée passée à interpret_result
    // :type Output:           Le type de la donnée retournée par interpret_result
    // :fn   mode_number:      Méthode retournant le numéro du mode pour lequel le pid est implémenté
    // :fn   pid_number:       Méthode retournant le numéro du pid implémenté
    // :fn   result_size:      Méthode retournant les tailles attendu pour le résultat en octets
    // :fn   description:      Description briève du pid
    // :fn   min:              Valeur min d'entrée
    // :fn   max:              Valeur max d'entrée
    // :fn   unit:             Unitée dans laquelle est retournée le résultat
    // :fn   interpret_result: Méthode retournant le résultat interprété d'une entrée donnée en paramètres
    //
    type Input;
    type Output;
    fn mode_number(&self) -> ModLen;
    fn pid_number(&self)  -> PidLen;
    fn result_size(&self) -> ResultSize;
    fn description(&self) -> &'static str;
    fn max(&self)         -> Option<Self::Output>;
    fn min(&self)         -> Option<Self::Output>;
    fn unit(&self)        -> Option<&'static str>;
    fn interpret_result(&self, input: Self::Input) -> Self::Output;
    fn to_string(&self) -> String {
        format!("Pid(mode={}, pid={}, result_size={:?})", self.mode_number(), self.pid_number(), self.result_size())
    }
}


// Mode 0x01
pub struct AvailablePids20;
impl AvailablePids20 { pub fn new() -> Self { AvailablePids20 } }
impl Pid for AvailablePids20 {
    type Input  = u32;
    type Output = Vec<PidLen>;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x00 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Pids supportés de 0x00 à 0x1f" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_available_pids(input, 0)
    }
}

pub struct StatusSinceDTC;
impl StatusSinceDTC { pub fn new() -> Self { StatusSinceDTC } }
impl Pid for StatusSinceDTC {
    type Input  = u32;
    type Output = u32;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x01 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Surveiller l'état depuis l'effacement des DTC. (Inclut l'état du témoin de dysfonctionnement (MIL) et le nombre de DTC.)" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        input
    }
}

pub struct FreezeDTC;
impl FreezeDTC { pub fn new() -> Self { FreezeDTC } }
impl Pid for FreezeDTC {
    type Input  = u16;
    type Output = ();
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x02 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Geler DTC" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, _: Self::Input) -> Self::Output {}
}

pub struct FuelSystemStatus;
impl FuelSystemStatus { 
    pub fn new() -> Self { FuelSystemStatus } }
impl Pid for FuelSystemStatus {
    type Input  = u16;
    type Output = (FuelSystem, FuelSystem);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x03 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Status du système de fuel" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_system(input)
    }
}

pub struct EngineLoad;
impl EngineLoad { pub fn new() -> Self { EngineLoad } }
impl Pid for EngineLoad {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x04 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Charge du véhicule" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(100.0) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_percent(input)
    }
}

pub struct EngineCoolantTemperature;
impl EngineCoolantTemperature { pub fn new() -> Self { EngineCoolantTemperature } }
impl Pid for EngineCoolantTemperature {
    type Input  = u8;
    type Output = i16;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x05 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Température du liquide de refroidissement du moteur" }
    fn min(&self)  -> Option<Self::Output>  { Some(-40) }
    fn max(&self)  -> Option<Self::Output>  { Some(215) }
    fn unit(&self) -> Option<&'static str> { Some("°C") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_celsius(input)
    }
}

pub struct ShortTermFuelTrim1;
impl ShortTermFuelTrim1 { pub fn new() -> Self { ShortTermFuelTrim1 } }
impl Pid for ShortTermFuelTrim1 {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x06 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Trim de carburant à court terme, banque 1" }
    fn min(&self)  -> Option<Self::Output>  { Some(-100.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(99.2) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_trim(input)
    }
}

pub struct LongTermFuelTrim1;
impl LongTermFuelTrim1 { pub fn new() -> Self { LongTermFuelTrim1 } }
impl Pid for LongTermFuelTrim1 {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x07 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Trim de carburant à long terme, banque 1" }
    fn min(&self)  -> Option<Self::Output>  { Some(-100.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(99.2) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_trim(input)
    }
}

pub struct ShortTermFuelTrim2;
impl ShortTermFuelTrim2 { pub fn new() -> Self { ShortTermFuelTrim2 } }
impl Pid for ShortTermFuelTrim2 {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x08 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Trim de carburant à court terme, banque 2" }
    fn min(&self)  -> Option<Self::Output>  { Some(-100.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(99.2) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_trim(input)
    }
}

pub struct LongTermFuelTrim2;
impl LongTermFuelTrim2 { pub fn new() -> Self { LongTermFuelTrim2 } }
impl Pid for LongTermFuelTrim2 {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x07 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Trim de carburant à long terme, banque 2" }
    fn min(&self)  -> Option<Self::Output>  { Some(-100.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(99.2) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_trim(input)
    }
}

pub struct FuelPressure;
impl FuelPressure { pub fn new() -> Self { FuelPressure } }
impl Pid for FuelPressure {
    type Input  = u8;
    type Output = u16;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x0a }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Pression du fuel" }
    fn min(&self)  -> Option<Self::Output>  { Some(0) }
    fn max(&self)  -> Option<Self::Output>  { Some(765) }
    fn unit(&self) -> Option<&'static str> { Some("kPa") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        input as u16 * 3
    }
}

pub struct IntakeManifoldAbsolutePressure;
impl IntakeManifoldAbsolutePressure { pub fn new() -> Self { IntakeManifoldAbsolutePressure } }
impl Pid for IntakeManifoldAbsolutePressure {
    type Input  = u8;
    type Output = u8;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x0b }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Pression absolue du collecteur d'admission" }
    fn min(&self)  -> Option<Self::Output>  { Some(0) }
    fn max(&self)  -> Option<Self::Output>  { Some(255) }
    fn unit(&self) -> Option<&'static str> { Some("kPa") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        input
    }
}

pub struct EngineSpeed;
impl EngineSpeed { pub fn new() -> Self { EngineSpeed } }
impl Pid for EngineSpeed {
    type Input  = u16;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x0c }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Vitesse du moteur" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(16383.75) }
    fn unit(&self) -> Option<&'static str> { Some("rpm") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_rpm(input)
    }
}

pub struct VehicleSpeed;
impl VehicleSpeed { pub fn new() -> Self { VehicleSpeed } }
impl Pid for VehicleSpeed {
    type Input  = u8;
    type Output = u8;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x0d }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Vitesse du véhicule" }
    fn min(&self)  -> Option<Self::Output>  { Some(0) }
    fn max(&self)  -> Option<Self::Output>  { Some(255) }
    fn unit(&self) -> Option<&'static str> { Some("km/h") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        input
    }
}

pub struct TimingAdvance;
impl TimingAdvance { pub fn new() -> Self { TimingAdvance } }
impl Pid for TimingAdvance {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x0e }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Avance de temps" }
    fn min(&self)  -> Option<Self::Output>  { Some(-64.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(63.5) }
    fn unit(&self) -> Option<&'static str> { Some("°before TDC") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_timing_advance(input)
    }
}

pub struct IntakeAirTemperature;
impl IntakeAirTemperature { pub fn new() -> Self { IntakeAirTemperature } }
impl Pid for IntakeAirTemperature {
    type Input  = u8;
    type Output = i16;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x0f }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Temperatur de l'air d'admission" }
    fn min(&self)  -> Option<Self::Output>  { Some(-40) }
    fn max(&self)  -> Option<Self::Output>  { Some(215) }
    fn unit(&self) -> Option<&'static str> { Some("°C") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_celsius(input)
    }
}

pub struct MAFSensor;
impl MAFSensor { pub fn new() -> Self { MAFSensor } }
impl Pid for MAFSensor {
    type Input  = u16;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x10 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Débit d'air du capteur de débit d'air massique (MAF)" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(655.35) }
    fn unit(&self) -> Option<&'static str> { Some("grams/sec") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_maf(input)
    }
}

pub struct ThrottlePosition;
impl ThrottlePosition { pub fn new() -> Self { ThrottlePosition } }
impl Pid for ThrottlePosition {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x11 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Position du papillon" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(100.0) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_percent(input)
    }
}

pub struct CommendedSecondaryAirStatus;
impl CommendedSecondaryAirStatus { pub fn new() -> Self { CommendedSecondaryAirStatus } }
impl Pid for CommendedSecondaryAirStatus {
    type Input  = u8;
    type Output = AirStatus;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x12 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Statut aérien secondaire commandé" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_air_status(input)
    }
}

pub struct OxygenSensorPresent;
impl OxygenSensorPresent { pub fn new() -> Self { OxygenSensorPresent } }
impl Pid for OxygenSensorPresent {
    type Input  = u8;
    type Output = ();
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x13 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen présents dans les deux banques" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, _: Self::Input) -> Self::Output {
        // TODO
        ()
    }
}

pub struct OxygenSensor1;
impl OxygenSensor1 { pub fn new() -> Self { OxygenSensor1 } }
impl Pid for OxygenSensor1 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x14 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 1, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor2;
impl OxygenSensor2 { pub fn new() -> Self { OxygenSensor2 } }
impl Pid for OxygenSensor2 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x15 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 2, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor3;
impl OxygenSensor3 { pub fn new() -> Self { OxygenSensor3 } }
impl Pid for OxygenSensor3 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x16 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 3, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor4;
impl OxygenSensor4 { pub fn new() -> Self { OxygenSensor4 } }
impl Pid for OxygenSensor4 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x17 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 4, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor5;
impl OxygenSensor5 { pub fn new() -> Self { OxygenSensor5 } }
impl Pid for OxygenSensor5 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x18 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 5, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor6;
impl OxygenSensor6 { pub fn new() -> Self { OxygenSensor6 } }
impl Pid for OxygenSensor6 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x19 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 6, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor7;
impl OxygenSensor7 { pub fn new() -> Self { OxygenSensor7 } }
impl Pid for OxygenSensor7 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x1a }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 7, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct OxygenSensor8;
impl OxygenSensor8 { pub fn new() -> Self { OxygenSensor8 } }
impl Pid for OxygenSensor8 {
    type Input  = u16;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x1b }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen 8, tension et garniture de carburant à court terme" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, -100.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((1.275, 99.2)) }
    fn unit(&self) -> Option<&'static str> { Some("(volts, %)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor(input)
    }
}

pub struct ObdStandardForThisVehicle;
impl ObdStandardForThisVehicle { pub fn new() -> Self { ObdStandardForThisVehicle } }
impl Pid for ObdStandardForThisVehicle {
    type Input  = u8;
    type Output = ObdStandard;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x1c }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Standard Obd suivi par cette voiture" }
    fn min(&self)  -> Option<Self::Output>  { Some(ObdStandard::Value(1)) }
    fn max(&self)  -> Option<Self::Output>  { Some(ObdStandard::Value(250)) }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_obd_standard(input)
    }
}

pub struct OxygenSensorPresent4Banks;
impl OxygenSensorPresent4Banks { pub fn new() -> Self { OxygenSensorPresent4Banks } }
impl Pid for OxygenSensorPresent4Banks {
    type Input  = u8;
    type Output = ();
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x1d }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Capteurs d'oxygen présents dans les quatres banques" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, _: Self::Input) -> Self::Output {
        // TODO
        ()
    }
}

pub struct AuxiliaryInputStatus;
impl AuxiliaryInputStatus { pub fn new() -> Self { AuxiliaryInputStatus } }
impl Pid for AuxiliaryInputStatus {
    type Input  = u8;
    type Output = State;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x1e }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "État de l'entrée auxiliaire" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_auxiliary_input_status(input)
    }
}

pub struct RunTimeSinceStart;
impl RunTimeSinceStart { pub fn new() -> Self { RunTimeSinceStart } }
impl Pid for RunTimeSinceStart {
    type Input  = u16;
    type Output = u16;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x1f }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Temps écoulé depuis l'allumage du véhicule" }
    fn min(&self)  -> Option<Self::Output>  { Some(0) }
    fn max(&self)  -> Option<Self::Output>  { Some(65535) }
    fn unit(&self) -> Option<&'static str> { Some("seconds") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_seconds(input)
    }
}

pub struct AvailablePids40;
impl AvailablePids40 { pub fn new() -> Self { AvailablePids40 } }
impl Pid for AvailablePids40 {
    type Input  = u32;
    type Output = Vec<PidLen>;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x20 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Pids supportés de 0x21 à 0x3f" }
    fn min(&self)  -> Option<Self::Output>  { None }
    fn max(&self)  -> Option<Self::Output>  { None }
    fn unit(&self) -> Option<&'static str> { None }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_available_pids(input, 1)
    }
}

pub struct DistanceWithMIL;
impl DistanceWithMIL { pub fn new() -> Self { DistanceWithMIL } }
impl Pid for DistanceWithMIL {
    type Input  = u16;
    type Output = u16;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x21 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Distance parcourue avec témoin de dysfonctionnement (MIL) allumé" }
    fn min(&self)  -> Option<Self::Output>  { Some(0) }
    fn max(&self)  -> Option<Self::Output>  { Some(65535) }
    fn unit(&self) -> Option<&'static str> { Some("km") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_km(input)
    }
}

pub struct FuelRailPressure;
impl FuelRailPressure { pub fn new() -> Self { FuelRailPressure } }
impl Pid for FuelRailPressure {
    type Input  = u16;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x22 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Pression de rampe de carburant (par rapport au vide du collecteur)" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(5177.265) }
    fn unit(&self) -> Option<&'static str> { Some("kPa") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_rail_pressure(input)
    }
}

pub struct FuelRailGaugePressure;
impl FuelRailGaugePressure { pub fn new() -> Self { FuelRailGaugePressure } }
impl Pid for FuelRailGaugePressure {
    type Input  = u16;
    type Output = u32;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x23 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x02) }
    fn description(&self) -> &'static str { "Pression de jauge de rampe de carburant (diesel ou injection directe d'essence)" }
    fn min(&self)  -> Option<Self::Output>  { Some(0) }
    fn max(&self)  -> Option<Self::Output>  { Some(655350) }
    fn unit(&self) -> Option<&'static str> { Some("kPa") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_fuel_rail_gauge_pressure(input)
    }
}

pub struct OxygenSensorLambda1;
impl OxygenSensorLambda1 { pub fn new() -> Self { OxygenSensorLambda1 } }
impl Pid for OxygenSensorLambda1 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x24 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 1, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda2;
impl OxygenSensorLambda2 { pub fn new() -> Self { OxygenSensorLambda2 } }
impl Pid for OxygenSensorLambda2 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x25 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 2, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda3;
impl OxygenSensorLambda3 { pub fn new() -> Self { OxygenSensorLambda3 } }
impl Pid for OxygenSensorLambda3 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x26 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 3, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda4;
impl OxygenSensorLambda4 { pub fn new() -> Self { OxygenSensorLambda4 } }
impl Pid for OxygenSensorLambda4 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x27 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 4, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda5;
impl OxygenSensorLambda5 { pub fn new() -> Self { OxygenSensorLambda5 } }
impl Pid for OxygenSensorLambda5 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x28 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 5, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda6;
impl OxygenSensorLambda6 { pub fn new() -> Self { OxygenSensorLambda6 } }
impl Pid for OxygenSensorLambda6 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x29 }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 6, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda7;
impl OxygenSensorLambda7 { pub fn new() -> Self { OxygenSensorLambda7 } }
impl Pid for OxygenSensorLambda7 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x2a }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 7, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct OxygenSensorLambda8;
impl OxygenSensorLambda8 { pub fn new() -> Self { OxygenSensorLambda8 } }
impl Pid for OxygenSensorLambda8 {
    type Input  = u32;
    type Output = (f64, f64);
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x2b }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x04) }
    fn description(&self) -> &'static str { "Capteur d'oxygène 8, AB : rapport d'équivalence air-carburant (lambda,λ), CD : Tension" }
    fn min(&self)  -> Option<Self::Output>  { Some((0.0, 0.0)) }
    fn max(&self)  -> Option<Self::Output>  { Some((2.0, 8.0)) }
    fn unit(&self) -> Option<&'static str> { Some("(ratio, volts)") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_oxygen_sensor_lambda(input)
    }
}

pub struct CommandedEGR;
impl CommandedEGR { pub fn new() -> Self { CommandedEGR } }
impl Pid for CommandedEGR {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x2c }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "EGR commandé" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(100.0) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_percent(input)
    }
}

pub struct EGRError;
impl EGRError { pub fn new() -> Self { EGRError } }
impl Pid for EGRError {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x2d }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Erreur EGR" }
    fn min(&self)  -> Option<Self::Output>  { Some(-100.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(99.2) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_egr_error(input)
    }
}

pub struct CommandedEvaporativePurge;
impl CommandedEvaporativePurge { pub fn new() -> Self { CommandedEvaporativePurge } }
impl Pid for CommandedEvaporativePurge {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x2e }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Purge par évaporation commandée" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(100.0) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_percent(input)
    }
}

pub struct FuelTankLevelInput;
impl FuelTankLevelInput { pub fn new() -> Self { FuelTankLevelInput } }
impl Pid for FuelTankLevelInput {
    type Input  = u8;
    type Output = f64;
    fn mode_number(&self) -> ModLen { 0x01 }
    fn pid_number(&self)  -> PidLen { 0x2f }
    fn result_size(&self) -> ResultSize { ResultSize::Value(0x01) }
    fn description(&self) -> &'static str { "Entrée de niveau de réservoir de carburant" }
    fn min(&self)  -> Option<Self::Output>  { Some(0.0) }
    fn max(&self)  -> Option<Self::Output>  { Some(100.0) }
    fn unit(&self) -> Option<&'static str> { Some("%") }
    fn interpret_result(&self, input: Self::Input) -> Self::Output {
        decode_percent(input)
    }
}