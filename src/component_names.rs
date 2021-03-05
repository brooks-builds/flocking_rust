#[derive(Eq, PartialEq, Hash)]
pub enum ComponentNames {
    Location,
    Velocity,
    Acceleration,
    Rotation,
}

impl Into<String> for ComponentNames {
    fn into(self) -> String {
        match self {
            ComponentNames::Location => "Location".to_owned(),
            ComponentNames::Velocity => "Velocity".to_owned(),
            ComponentNames::Acceleration => "Acceleration".to_owned(),
            ComponentNames::Rotation => "Rotation".to_owned(),
        }
    }
}
