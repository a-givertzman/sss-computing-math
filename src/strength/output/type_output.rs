#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeOutput {
    LightweightTonnageIntensity,
    DeadweightIntensity,
    DisplacementIntensity,
    BuoyantIntensity,
    TotalLoadIntensity,
    ShearForce,
    BendingMoment,
    Stress,
}