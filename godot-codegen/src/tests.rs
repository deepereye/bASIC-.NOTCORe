
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::util::{to_pascal_case, to_snake_case};

#[test]
fn test_pascal_conversion() {
    // More in line with Rust identifiers, and eases recognition of other automation (like enumerator mapping).
    #[rustfmt::skip]
    let mappings = [
                                 ("AABB", "Aabb"),
                           ("AESContext", "AesContext"),
                              ("AStar3D", "AStar3D"),
                      ("AudioEffectEQ21", "AudioEffectEq21"),
                       ("AudioStreamWAV", "AudioStreamWav"),
                      ("CharFXTransform", "CharFxTransform"),
                       ("CPUParticles3D", "CpuParticles3D"),
              ("EditorSceneImporterGLTF", "EditorSceneImporterGltf"),
                              ("GIProbe", "GiProbe"),
                          ("HMACContext", "HmacContext"),
                           ("HSeparator", "HSeparator"),