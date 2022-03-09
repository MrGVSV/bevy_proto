use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtoError {
    #[error("could not find a type registration for {name}")]
    NotRegistered { name: String },
    #[error("could not reflect `ProtoComponent` for {name}. Did you add a `#[reflect(ProtoComponent)]` to your type?")]
    MissingReflection { name: String },
    #[error("could not reflect {name}")]
    BadReflection { name: String },
}
