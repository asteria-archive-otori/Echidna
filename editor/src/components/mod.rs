/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod editor;
pub mod sidebar;
// pub mod tab_label;
pub mod window;
pub use editor::EchidnaCoreEditor;
pub use sidebar::EchidnaSidebar;
// pub use tab_label::TabLabel;
pub use window::EchidnaWindow;

pub mod prelude {
    pub use super::window::{file::*, menubar::*};
}
