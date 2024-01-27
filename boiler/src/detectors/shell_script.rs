use crate::data::Repo;
use crate::detectors::{Detector, DetectorResult};
use crate::detectors_utils::detect_by_extension;

pub struct ShellScriptDetector;

impl Detector for ShellScriptDetector {
    fn detect(&self, repo: &Repo) -> DetectorResult {
        detect_by_extension(repo, &["sh"], "shell")
    }
}

// value = b"#!/usr/local/bin/bash"
// value = b"#!/usr/local/bin/fish"
// value = b"#!/usr/local/bin/tcsh"
// value = b"#!/usr/local/bin/ash"
// value = b"#!/usr/local/bin/zsh"
// value = b"#!/usr/bin/env bash"
// value = b"#!/usr/bin/env fish"
// value = b"#!/usr/bin/env zsh"
// value = b"#!/usr/local/bash"
// value = b"#!/usr/local/tcsh"
// value = b"#!/usr/bin/bash"
// value = b"#!/usr/bin/fish"
// value = b"#!/usr/bin/tcsh"
// value = b"#!/usr/bin/zsh"
// value = b"#!/bin/bash"
// value = b"#!/bin/tcsh"
// value = b"#!/bin/ash"
// value = b"#!/bin/csh"
// value = b"#!/bin/ksh"
// value = b"#!/bin/zsh"
// value = b"#!/bin/sh"
