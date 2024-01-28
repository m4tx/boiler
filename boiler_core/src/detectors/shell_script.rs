use boiler_macros::FunctionMeta;

use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_header;

/// Detects if the project contains shell scripts.
#[derive(Debug, FunctionMeta)]
pub struct ShellScriptDetector;

impl Detector for ShellScriptDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_header(repo, &SHELLSCRIPT_SHEBANGS, "shell")
    }
}

const SHELLSCRIPT_SHEBANGS: [&[u8]; 21] = [
    b"#!/usr/local/bin/bash",
    b"#!/usr/local/bin/fish",
    b"#!/usr/local/bin/tcsh",
    b"#!/usr/local/bin/ash",
    b"#!/usr/local/bin/zsh",
    b"#!/usr/bin/env bash",
    b"#!/usr/bin/env fish",
    b"#!/usr/bin/env zsh",
    b"#!/usr/local/bash",
    b"#!/usr/local/tcsh",
    b"#!/usr/bin/bash",
    b"#!/usr/bin/fish",
    b"#!/usr/bin/tcsh",
    b"#!/usr/bin/zsh",
    b"#!/bin/bash",
    b"#!/bin/tcsh",
    b"#!/bin/ash",
    b"#!/bin/csh",
    b"#!/bin/ksh",
    b"#!/bin/zsh",
    b"#!/bin/sh",
];

#[cfg(test)]
mod tests {
    use crate::context_keys;
    use crate::data::Value;
    use crate::detectors::shell_script::ShellScriptDetector;
    use crate::detectors::Detector;
    use crate::test_utils::TempRepo;

    #[test]
    fn test_detect_shell_script() {
        let temp_repo = TempRepo::new();
        temp_repo.write_str("manage", "#!/usr/bin/bash\n\necho \"Hello, world!\"");

        let detector = ShellScriptDetector;
        let data = detector.detect(&temp_repo.repo()).unwrap();

        assert_eq!(
            data,
            Value::new_object([(
                context_keys::LANGS.to_owned(),
                Value::new_array(vec![Value::new_string("shell")])
            ),])
        );
    }
}
