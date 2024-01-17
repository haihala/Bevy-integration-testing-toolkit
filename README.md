# Bevy integration testing tool (Bitt)

Tool for integration testing apps built in the bevy game engine.

Bitt, (Noun) a pair of posts on the deck of a ship for fastening mooring lines or cables.

⚠️ Very much a work in progress.

## How to use

At the moment, only record/playback testing is implemented. To use it, add a system to your app that calls
`bitt::Asserter::pass` when a test case is passed. Also add the `bitt::PlaybackTestGear` plugin with the
name of the test case you want to use. Your inputs and artefacts will be saved under this name in
`bitt/test_scripts/<script name>.bitt_script` and `bitt/artefacts/<script name>` respectively.

If you then launch the game, it should run normally until `bitt::Asserter::pass` is called, at which point
it will save the inputs. On a subsequent run, it will load the inputs and replay them. If the asserter doesn't
pass after the inputs are done, it panics which causes a non-zero return code, which can be used to fail a CI.
There is a bit of wiggle room for when the asserters are checked. A screenshot is saved in the artefacts folder
both before and after this window.

For examples, see:

- `crates/demo_game/bin/integration_test.rs` for how to use the input recording and playback.
- `.github/workflows/commit-validation.yml` for how to run the integration tests in a github action.
    - Including how to make the screenshots visible in the github action logs.

Warnings:

- Currently, natural inputs **are** still listened to while in playback mode. This can cause tests to fail when
  they shouldn't. This will be fixed in the future.
- Running things in CI may not work exactly like it does locally.
  - Github action runner can't play audio, which may cause differing behavior.
  - CI may run at a different, usually lower framerate
- Ironically, the framwork isn't that well tested as of yet. It's possible that it may not work on your system.
  If you have any issues, please open an issue on github.

Recommendations:

- Put `bitt/test_scripts` in the repo, but gitignore `bitt/artefacts`.
- Use `cargo-make` to run the integration tests
- Use `clap` to parse command line arguments instead of using env vars to select test cases
