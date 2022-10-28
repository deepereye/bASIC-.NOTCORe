# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

extends Node

func _ready():
	var allow_focus := true
	var unrecognized_args: Array = []
	for arg in OS.get_cmdline_user_args():
		match arg:
			"--disallow-focus":
				allow_focus = false
			_:
				unrecognized_args.push_back(arg)

	if unrecognized_args:
		push_error("Unrecognized arguments: ", unrecognized