/*
 * services/view/options.rs
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

use wikidot_path::{ArgumentSchema, ArgumentValue, PageArguments};

const PAGE_ARGUMENTS_SCHEMA: ArgumentSchema = ArgumentSchema {
    valid_keys: &[
        "edit",
        "title",
        "parentPage",
        "parent",
        "tags",
        "noredirect",
        "norender",
        "comments",
        "discuss",
        "history",
        "offset",
        "data",
    ],
    solo_keys: &[
        "edit",
        "noredirect",
        "norender",
        "comments",
        "discuss",
        "history",
    ],
};

/// Describes the operations to be done by Framerail for this page view.
///
/// `RawPageOptions` (from `wikidot_path`) contains the direct key / value
/// pairs, this struct parses them into logical flags to be processed.
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PageOptions {
    edit: bool,
    title: Option<String>,
    parent: Option<String>,
    tags: Option<String>,
    no_redirect: bool,
    no_render: bool,
    comments: bool,
    history: bool,
    offset: Option<i32>,
    data: String,
}

impl PageOptions {
    pub fn parse(extra: &str) -> Self {
        tide::log::info!("Parsing page options: '{extra}'");

        let mut arguments = PageArguments::parse(extra, PAGE_ARGUMENTS_SCHEMA).0;
        let mut options = PageOptions::default();

        macro_rules! set_bool {
            ($field:ident, $key:ident $(,)?) => {{
                if let Some((value, _)) = arguments.remove(stringify!($key)) {
                    options.$field = to_bool(value);
                }
            }};
            ($field:ident $(,)?) => {
                set_bool!($field, $field)
            };
        }

        macro_rules! set_str {
            ($field:ident, $key:ident $(,)?) => {{
                if let Some((_, value)) = arguments.remove(stringify!($key)) {
                    options.$field = str!(value);
                }
            }};
            ($field:ident $(,)?) => {
                set_str!($field, $field)
            };
        }

        macro_rules! set_str_opt {
            ($field:ident, $key:ident $(,)?) => {{
                if let Some((_, value)) = arguments.remove(stringify!($key)) {
                    options.$field = Some(str!(value));
                }
            }};
            ($field:ident $(,)?) => {
                set_str_opt!($field, $field)
            };
        }

        set_bool!(edit);
        set_str_opt!(title);
        set_str_opt!(parent);
        set_str_opt!(parent, parentPage);
        set_str_opt!(tags);
        set_bool!(no_redirect, noredirect);
        set_bool!(no_render, norender);
        set_bool!(comments);
        set_bool!(comments, discuss);
        set_bool!(history);

        if let Some((value, orig)) = arguments.remove("offset") {
            match value {
                ArgumentValue::Integer(offset) => options.offset = Some(offset),
                _ => tide::log::error!("Invalid value for offset argument: {orig}"),
            }
        }

        set_str!(data);

        options
    }
}

fn to_bool(value: ArgumentValue) -> bool {
    tide::log::debug!("Converting argument value to plain boolean: {value:?}");

    match value {
        // Simply unwrap bool
        ArgumentValue::Boolean(b) => b,

        // Convert integer to truthy/falsey
        ArgumentValue::Integer(i) => i != 0,

        // Presence indicates a value of true
        ArgumentValue::String(_) | ArgumentValue::Null => true,
    }
}
