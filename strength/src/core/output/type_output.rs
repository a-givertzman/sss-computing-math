#[derive(Debug, Clone, Copy)]
pub enum TypeOutput {
    LightweghtIntensity,
    DeadweightIntensity,
    DisplacementTonnageIntensity,
    BuoyantIntensity,
    BendingMoment,
    ShearForce,
    Stress,
}