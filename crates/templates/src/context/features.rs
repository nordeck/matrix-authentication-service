// Copyright 2024 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use minijinja::{value::StructObject, Value};

/// Site features information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiteFeatures {
    /// Whether local password-based registration is enabled.
    pub password_registration: bool,

    /// Whether local password-based login is enabled.
    pub password_login: bool,
}

impl StructObject for SiteFeatures {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "password_registration" => Some(Value::from(self.password_registration)),
            "password_login" => Some(Value::from(self.password_login)),
            _ => None,
        }
    }

    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["password_registration", "password_login"])
    }
}
