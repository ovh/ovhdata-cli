use thiserror::Error as ThisError;

// Named ParseResult instead of Result because it is typically used alongside structures that
// derive Clap's Parser trait using a macro which expect Result to be the unaltered std version
pub type ParseResult<T> = Result<T, ParseError>;

// Same as ParseResult, not using the `Error` name to avoid conflicting with all the code that
// ask for a standard Error, such as the Clap macros
#[derive(ThisError, Debug)]
pub enum ParseError {
    #[error("Invalid format, must be formatted like name=value")]
    NameValueParse,
    #[error("Invalid volume, must be formatted like container@alias(/prefix):mount_path(:permission)(:cache) or url:mount_path(:permission)(:cache) or standalone:mount_path(:permission)")]
    JobVolumeParse,
    #[error("Invalid volume permission")]
    JobVolumePermission,
    #[error("Invalid output format")]
    OutputParse,
    #[error("Invalid role format")]
    RoleParse,
    #[error("Invalid container format, must not contains '/'")]
    ContainerParse,
}
