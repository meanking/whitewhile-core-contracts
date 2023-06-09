use cosmwasm_std::StdError;
use semver::Version;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Semver parsing error: {0}")]
    SemVer(String),

    #[error("Attempt to migrate to version {new_version}, but contract is on a higher version {current_version}")]
    MigrateInvalidVersion {
        new_version: Version,
        current_version: Version,
    },

    #[error("The provided assets are both the same")]
    SameAsset {},

    #[error("{asset} is invalid")]
    InvalidAsset { asset: String },

    #[error("Pair already exist")]
    ExistingPair {},

    #[error("Pair doesn't exist")]
    UnExistingPair {},

    #[error("Trio already exist")]
    ExistingTrio {},

    #[error("Trio doesn't exist")]
    NonExistantTrio {},

    #[error("A balance greater than zero is required by the factory to verify the asset")]
    InvalidVerificationBalance {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Attempt to migrate a pool to a different type. Make sure you are using the write Migrate message, e.g. MigratePair or MigrateTrio for pairs and trios respectively.")]
    MigratingWrongPool {},
}

impl From<semver::Error> for ContractError {
    fn from(err: semver::Error) -> Self {
        Self::SemVer(err.to_string())
    }
}
