#[derive(Eq, PartialEq, Hash)]
pub enum ResourceNames {
    BackgroundColor,
    BirdMesh,
    ArenaSize,
    UpdateFps,
    SightRange,
    AvoidRange,
    TurningSpeed,
    AttractionTurningSpeed,
    BoidColor,
    ColorChangeRate,
    ColorChangeSpeed,
    ClearScreenMesh,
}

impl Into<String> for ResourceNames {
    fn into(self) -> String {
        match self {
            ResourceNames::BackgroundColor => "BackgroundColor".to_owned(),
            ResourceNames::BirdMesh => "BirdMesh".to_owned(),
            ResourceNames::ArenaSize => "ArenaSize".to_owned(),
            ResourceNames::UpdateFps => "UpdateFps".to_owned(),
            ResourceNames::SightRange => "SightRange".to_owned(),
            ResourceNames::AvoidRange => "AvoidRange".to_owned(),
            ResourceNames::TurningSpeed => "TurningSpeed".to_owned(),
            ResourceNames::AttractionTurningSpeed => "AttractionTurningSpeed".to_owned(),
            ResourceNames::BoidColor => "BoidColor".to_owned(),
            ResourceNames::ColorChangeRate => "ColorChangeRate".to_owned(),
            ResourceNames::ColorChangeSpeed => "ColorChangeSpeed".to_owned(),
            ResourceNames::ClearScreenMesh => "ClearScreenMesh".to_owned(),
        }
    }
}

impl Into<&'static str> for ResourceNames {
    fn into(self) -> &'static str {
        match self {
            ResourceNames::BackgroundColor => "BackgroundColor",
            ResourceNames::BirdMesh => "BirdMesh",
            ResourceNames::ArenaSize => "ArenaSize",
            ResourceNames::UpdateFps => "UpdateFps",
            ResourceNames::SightRange => "SightRange",
            ResourceNames::AvoidRange => "AvoidRange",
            ResourceNames::TurningSpeed => "TurningSpeed",
            ResourceNames::AttractionTurningSpeed => "AttractionTurningSpeed",
            ResourceNames::BoidColor => "BoidColor",
            ResourceNames::ColorChangeRate => "ColorChangeRate",
            ResourceNames::ColorChangeSpeed => "ColorChangeSpeed",
            ResourceNames::ClearScreenMesh => "ClearScreenMesh",
        }
    }
}
