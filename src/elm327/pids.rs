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
    fn description(&self) -> &'static str { "Pids supportés de 0x00 à 0x20" }
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
        input as f64 / 2.55
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
        input as f64 / 1.28 - 100.0
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
        input as f64 / 1.28 - 100.0
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
        input as f64 / 1.28 - 100.0
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
        input as f64 / 1.28 - 100.0
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
        ( ((input>>8)*256) + (input&0xff) ) as f64 / 4.0
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
        input as f64 / 2.0 - 64.0
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