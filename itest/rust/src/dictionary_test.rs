/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::{HashMap, HashSet};

use crate::{expect_panic, itest};
use godot::builtin::{dict, varray, Dictionary, FromVariant, ToVariant, Variant};
use godot::obj::Share;

#[itest]
fn dictionary_default() {
    assert_eq!(Dictionary::default().len(), 0);
}

#[itest]
fn dictionary_new() {
    assert_eq!(Dictionary::new().len(), 0);
}

#[itest]
fn dictionary_from_iterator() {
    let dictionary = Dictionary::from_iter([("foo", 1), ("bar", 2)]);

    assert_eq!(dictionary.len(), 2);
    assert_eq!(dictionary.get("foo"), Some(1.to_variant()), "key = \"foo\"");
    assert_eq!(dictionary.get("bar"), Some(2.to_variant()), "key = \"bar\"");

    let dictionary = Dictionary::from_iter([(1, "foo"), (2, "bar")]);

    assert_eq!(dictionary.len(), 2);
    assert_eq!(dictionary.get(1), Some("foo".to_variant()), "key = 1");
    assert_eq!(dictionary.get(2), Some("bar".to_variant()), "key = 2");
}

#[itest]
fn dictionary_from() {
    let dictionary = Dictionary::from(&HashMap::from([("foo", 1), ("bar", 2)]));

    assert_eq!(dictionary.len(), 2);
    assert_eq!(dictionary.get("foo"), Some(1.to_variant()), "key = \"foo\"");
    assert_eq!(dictionary.get("bar"), Some(2.to_variant()), "key = \"bar\"");

    let dictionary = Dictionary::from(&HashMap::from([(1, "foo"), (2, "bar")]));

    assert_eq!(dictionary.len(), 2);
    assert_eq!(dictionary.get(1), Some("foo".to_variant()), "key = \"foo\"");
    assert_eq!(dictionary.get(2), Some("bar".to_variant()), "key = \"bar\"");
}

#[itest]
fn dictionary_macro() {
    let dictionary = dict! {
        "foo": 0,
        "bar": true,
        "baz": "foobar"
    };

    assert_eq!(dictionary.len(), 3);
    assert_eq!(dictionary.get("foo"), Some(0.to_variant()), "key = \"foo\"");
    assert_eq!(
        dictionary.get("bar"),
        Some(true.to_variant()),
        "key = \"bar\""
    );
    assert_eq!(
        dictionary.get("baz"),
        Some("foobar".to_variant()),
        "key = \"baz\""
    );

    let empty = dict!();
    assert!(empty.is_empty());

    let foo = "foo";
    let dict_complex = dict! {
        foo: 10,
        "bar": true,
        (1 + 2): Variant::nil(),
    };
    assert_eq!(dict_complex.get("foo"), Some(10.to_variant()));
    assert_eq!(dict_complex.get("bar"), Some(true.to_variant()));
    assert_eq!(dict_complex.get(3), Some(Variant::nil()));
}

#[itest]
fn dictionary_clone() {
    let subdictionary = dict! {
        "baz": true,
        "foobar": false
    };
    let dictionary = dict! {
        "foo": 0,
        "bar": subdictionary.share()
    };
    #[allow(clippy::redundant_c