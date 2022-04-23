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

use crate::internal::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct TabPage {}

#[glib::object_subclass]
impl ObjectSubclass for TabPage {
    const NAME: &'static str = "EchTabPage";

    type Type = super::TabPage;

    type ParentType = glib::Object;
}

impl ObjectImpl for TabPage {}
