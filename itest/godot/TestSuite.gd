# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

class_name TestSuite
extends RefCounted

var _assertion_failed: bool = false

## Asserts that `what` is `true`, but does not abort the test. Returns `what` so you can return
## early from the test function if the assertion 