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
		push_error("Unrecognized arguments: ", unrecognized_args)
		get_tree().quit(2)
		return

	var rust_runner = IntegrationTests.new()

	var gdscript_suites: Array = [
		preload("res://ManualFfiTests.gd").new(),
		preload("res://gen/GenFfiTests.gd").new(),
	]
	
	var gdscript_tests: Array = []
	for suite in gdscript_suites:
		for method in suite.get_method_list():
			var method_name: String = method.name
			if method_name.begins_with("test_"):
				gdscript_tests.push_back(GDScriptTestCase.new(suite, 