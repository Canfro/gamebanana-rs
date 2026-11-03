use std::marker::PhantomData;

use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, DeserializeUntaggedVerboseError)]
#[serde(deny_unknown_fields)]
pub enum Response<T> {
    Ok(T),
    Err {
        #[serde(skip_deserializing)]
        _marker: PhantomData<T>,

        #[serde(rename = "_sErrorCode")]
        error_code: String,

        #[serde(rename = "_sErrorMessage")]
        error_message: String,
    },
}
