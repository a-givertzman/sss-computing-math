#[derive(Debug, Clone, Copy)]
pub enum TypeOutput {
    LightweightIntensity,
    DeadweightIntensity,
    DisplacementIntensity,
    BuoyantIntensity,
    TotalLoadIntensity,
    ShearForce,
    BendingMoment,
    Stress,
}