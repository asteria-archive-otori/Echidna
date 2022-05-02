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
mod imp;

glib::wrapper! {
    pub struct TabPage(ObjectSubclass<imp::TabPage>);
}
