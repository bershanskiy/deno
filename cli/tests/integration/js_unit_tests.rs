// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use test_util as util;

#[test]
fn js_unit_tests_lint() {
  let status = util::deno_cmd()
    .arg("lint")
    .arg("--unstable")
    .arg(util::tests_path().join("unit"))
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
  assert!(status.success());
}

#[test]
fn js_unit_tests() {
  let _g = util::http_server();

  // Note that the unit tests are not safe for concurrency and must be run with a concurrency limit
  // of one because there are some chdir tests in there.
  // TODO(caspervonb) split these tests into two groups: parallel and serial.
  let mut deno = util::deno_cmd()
    .current_dir(util::root_path())
    .arg("test")
    .arg("--unstable")
    // Flash tests are crashing with SIGSEGV on Ubuntu, so we'll disable these entirely
    .arg("--ignore=./cli/tests/unit/flash_test.ts")
    .arg("--location=http://js-unit-tests/foo/bar")
    .arg("--no-prompt")
    .arg("-A")
    .arg(util::tests_path().join("unit"))
    .spawn()
    .expect("failed to spawn script");

  let status = deno.wait().expect("failed to wait for the child process");
  #[cfg(unix)]
  assert_eq!(
    std::os::unix::process::ExitStatusExt::signal(&status),
    None,
    "Deno should not have died with a signal"
  );
  assert_eq!(Some(0), status.code(), "Deno should have exited cleanly");
  assert!(status.success());
}
