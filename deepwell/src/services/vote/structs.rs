/*
 * services/vote/structs.rs
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

pub type VoteValue = i16;

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateVote {
    pub page_id: i64,
    pub user_id: i64,
    pub value: VoteValue,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVote {
    pub page_id: i64,
    pub user_id: i64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase", tag = "type", content = "id")]
pub enum VoteHistoryKind {
    Page(i64),
    User(i64),
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetVoteHistory {
    #[serde(flatten)]
    pub kind: VoteHistoryKind,
    pub start_id: i64,
    pub deleted: Option<bool>,
    pub disabled: Option<bool>,
    pub limit: u64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CountVoteHistory {
    #[serde(flatten)]
    pub kind: VoteHistoryKind,
    pub start_id: i64,
    pub deleted: Option<bool>,
    pub disabled: Option<bool>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VoteAction {
    pub page_id: i64,
    pub user_id: i64,
    pub enable: bool,
    pub acting_user_id: i64,
}
