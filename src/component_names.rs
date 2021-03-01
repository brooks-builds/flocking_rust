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
            ComponentNames::Location => "location".to_owned(),
            ComponentNames::Velocity => "velocity".to_owned(),
            ComponentNames::Acceleration => "acceleration".to_owned(),
            ComponentNames::Rotation => "rotation".to_owned(),
        }
    }
}

impl Into<&str> for ComponentNames {
    fn into(self) -> &'static str {
        match self {
            ComponentNames::Location => "Location",
            ComponentNames::Velocity => "Velocity",
            ComponentNames::Acceleration => "Acceleration",
            ComponentNames::Rotation => "Rotation",
        }
    }
}
