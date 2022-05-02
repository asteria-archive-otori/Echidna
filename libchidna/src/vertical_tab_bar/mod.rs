/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2022 to Echidna Code contributors. All Rights Reserved
 *
 * List of authors:
 * Nefo Fortressia <nefothingy@hotmail.com>
 *
 */

mod imp;
pub mod model;
pub mod page;
use adw::TabView;

use crate::internal::prelude::*;

glib::wrapper! {
    ///
    /// A widget that displays the tabs of a TabView in a vertical manner
    ///
    pub struct VerticalTabBar(ObjectSubclass<imp::VerticalTabBar>)
    @extends adw::Bin, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl VerticalTabBar {
    pub fn new(tab_view: &TabView) -> Self {
        glib::Object::new(&[("tab-view", tab_view)]).unwrap()
    }

    pub fn tab_view(&self) -> TabView {
        self.property("tab-view")
    }
}
