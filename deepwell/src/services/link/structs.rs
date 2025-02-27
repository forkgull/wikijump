/*
 * services/link/structs.rs
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

use crate::models::page_connection::Model as PageConnectionModel;
use crate::models::page_connection_missing::Model as PageConnectionMissingModel;
use crate::models::page_link::Model as PageLinkModel;
use crate::web::Reference;
use time::OffsetDateTime;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksFrom<'a> {
    pub site_id: i64,
    pub page: Reference<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksFromOutput {
    pub present: Vec<PageConnectionModel>,
    pub absent: Vec<PageConnectionMissingModel>,
    pub external: Vec<PageLinkModel>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksTo<'a> {
    pub site_id: i64,
    pub page: Reference<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksToOutput {
    pub connections: Vec<PageConnectionModel>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksToMissing {
    pub site_id: i64,
    pub page_slug: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksToMissingOutput {
    pub connections: Vec<PageConnectionMissingModel>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetConnectionsFromOutput {
    pub present: Vec<PageConnectionModel>,
    pub absent: Vec<PageConnectionMissingModel>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksExternalFrom<'a> {
    pub site_id: i64,
    pub page: Reference<'a>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksExternalFromOutput {
    pub links: Vec<PageLinkModel>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksExternalTo {
    pub site_id: i64,
    pub url: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLinksExternalToOutput {
    pub links: Vec<ToExternalLink>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ToExternalLink {
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub page_id: i64,
    pub count: i32,
}
