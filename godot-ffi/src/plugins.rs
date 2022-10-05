/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/// Distributed self-registration of "plugins" without central list

// Note: code in this file is safe, however it seems that some annotations fall into the "unsafe" category.
// For example, adding #![forbid(unsafe_code)] causes this error:
//   note: the progra