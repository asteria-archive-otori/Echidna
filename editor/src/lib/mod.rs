/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod closeable_tab;
pub mod to_imp;
pub mod prelude {
    pub use super::closeable_tab::*;
    pub use super::to_imp::*;
}
