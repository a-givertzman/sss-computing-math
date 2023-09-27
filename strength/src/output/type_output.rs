#[derive(Debug, Clone, Copy)]
pub enum TypeOutput {
    LightweightIntensity,
    DeadweightIntensity,
    DisplacementTonnageIntensity,
    BuoyantIntensity,
    BendingMoment,
    ShearForce,
    Stress,
}