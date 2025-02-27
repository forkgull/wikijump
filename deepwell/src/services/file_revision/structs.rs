/*
 * services/file_revision/structs.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use crate::services::page_revision::PageRevisionCountOutput;
use crate::web::FetchDirection;

#[derive(Debug)]
pub struct CreateFileRevision {
    pub site_id: i64,
    pub page_id: i64,
    pub file_id: i64,
    pub user_id: i64,
    pub comments: String,
    pub body: CreateFileRevisionBody,
}

#[derive(Debug, Default)]
pub struct CreateFileRevisionBody {
    pub page_id: ProvidedValue<i64>, // for changing the page this file is on
    pub name: ProvidedValue<String>,
    pub blob: ProvidedValue<FileBlob>,
    pub licensing: ProvidedValue<serde_json::Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileBlob {
    pub s3_hash: BlobHash,
    pub size_hint: i64,
    pub mime_hint: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CreateFileRevisionOutput {
    pub file_revision_id: i64,
    pub file_revision_number: i32,
}

#[derive(Debug)]
pub struct CreateFirstFileRevision {
    pub site_id: i64,
    pub page_id: i64,
    pub file_id: i64,
    pub user_id: i64,
    pub name: String,
    pub s3_hash: BlobHash,
    pub size_hint: i64,
    pub mime_hint: String,
    pub licensing: serde_json::Value,
    pub comments: String,
}

#[derive(Debug, Default)]
pub struct CreateFirstFileRevisionOutput {
    pub file_id: i64,
    pub file_revision_id: i64,
}

#[derive(Debug)]
pub struct CreateTombstoneFileRevision {
    pub site_id: i64,
    pub page_id: i64,
    pub file_id: i64,
    pub user_id: i64,
    pub comments: String,
}

#[derive(Debug)]
pub struct CreateResurrectionFileRevision {
    pub site_id: i64,
    pub page_id: i64,
    pub file_id: i64,
    pub user_id: i64,
    pub new_page_id: i64,
    pub new_name: String,
    pub comments: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFileRevision {
    pub page_id: i64,
    pub file_id: i64,
    pub revision_number: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFileRevision {
    pub page_id: i64,
    pub file_id: i64,
    pub revision_id: i64,
    pub user_id: i64,
    pub hidden: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFileRevisionRange {
    pub page_id: i64,
    pub file_id: i64,
    pub revision_number: i32,
    pub revision_direction: FetchDirection,
    pub limit: u64,
}

pub type FileRevisionCountOutput = PageRevisionCountOutput;
