# parameterized_tests
[![Build Status](https://travis-ci.org/icasdri/parameterized_tests.svg?branch=master)](https://travis-ci.org/icasdri/parameterized_tests)
[![Crate Version](https://meritbadge.herokuapp.com/rfc1751)](https://crates.io/crates/parameterized_tests)

Simple, flexible, and concise parameterized test macro for [Rust](https://rust-lang.org) testing. Inspired by JUnit's [`Parameterized` runner](https://github.com/junit-team/junit4/wiki/parameterized-tests) which separated testing data from testing logic. This allows, among other things like general [DRY](https://en.wikipedia.org/wiki/DRY_principle), many data-driven tests to be specified all at once.
