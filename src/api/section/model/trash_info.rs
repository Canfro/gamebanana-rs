use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;
use serde::{Deserialize, Serialize};

use crate::api::model::submitter::Submitter;

#[derive(Debug, Clone, Serialize, DeserializeUntaggedVerboseError)]
#[serde(deny_unknown_fields)]
pub enum TrashInfo {
    A {
        #[serde(rename = "_sReason")]
        reason: String,

        #[serde(rename = "_tsTrashDate")]
        trash_date: i64,

        #[serde(rename = "_bIsNewTrashSystem")]
        is_new_trash_system: bool,

        #[serde(rename = "_bIsTrashedByOwner")]
        is_trashed_by_owner: bool,
    },
    B {
        #[serde(rename = "_tsDateTrashed")]
        date_trashed: i64,

        #[serde(rename = "_bHideReason")]
        hide_reason: bool,

        #[serde(rename = "_aTrasher")]
        trasher: Submitter,

        #[serde(rename = "_bIsTrashedByOwner")]
        is_trashed_by_owner: bool,

        #[serde(rename = "_bIsNewTrashSystem")]
        is_new_trash_system: bool,
    },
}
