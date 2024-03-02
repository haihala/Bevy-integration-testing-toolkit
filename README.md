# Bevy integration testing tool (Bitt)

Tool for integration testing apps built in the bevy game engine.

Bitt, (Noun) a pair of posts on the deck of a ship for fastening mooring lines or cables.

[Documentation](https://docs.rs/bitt/) [crates.io](https://crates.io/crates/bitt)

## How to use

At the moment, only record/playback testing is implemented. To use it, add a system to your app that calls
`bitt::Asserter::pass` when a test case is passed. Also add the `bitt::PlaybackTestGear` plugin with the
name of the test case you want to use and a `bitt::PlaybackTestingOptions` that configures the test gear.
Your inputs and artefacts will be saved under this name in
`bitt/test_scripts/<script name>.bitt_script` and `bitt/artefacts/<script name>` respectively.

If you then launch the game, it should run normally until `bitt::Asserter::pass` is called, at which point
it will save the inputs. On a subsequent run, it will load the inputs and replay them. If the asserter doesn't
pass after the inputs are done, it panics which causes a non-zero return code, which can be used to fail a CI.
There is a bit of wiggle room for when the asserters are checked. A screenshot is saved in the artefacts folder
both before and after this window.

For examples, see:

- `crates/star_demo/src/bin/star_test.rs` for how to use the input recording and playback for keyboard/controller inputs.
  - Uses env vars to select the test case to run and other parameters.
  - Separate binary file for tests.
- `crates/click_demo/src/main.rs` for a different approach to playback testing a mouse driven game.
  - Uses clap to select the test case to run and other parameters.
  - Same binary file for tests and normal runs.
- `.github/workflows/commit-validation.yml` for how to run the integration tests in a github action.
    - Including how to make the artefacts visible in the github action logs.

Warnings and caveats:

- Currently, natural inputs **are** still listened to while in playback mode. This can cause tests to fail when
  they shouldn't. This will be fixed in the future.
- Running things in CI may not work exactly like it does locally.
  - Github action runner can't play audio, which may cause differing behavior.
  - CI may run at a different, usually lower framerate
- Ironically, the framwork isn't that well tested as of yet. It's possible that it may not work on your system.
  If you have any issues, please open an issue on github.
- Bevy stores mouse position in the window. This means that any tests that care about mouse movements will likely
  not work in headless mode.

Recommendations:

- Put `bitt/test_scripts` in the repo, but gitignore `bitt/artefacts`.
- Use `cargo-make` to run the integration tests
- Use `clap` to parse command line arguments instead of using env vars to select test cases

# Feedback and contributing

Feedback and contributions are welcome. Please open an issue on github if you have any feedback or suggestions. Some ideas I've had:

- Parallelize headless tests
- Inbuilt sharding method so that several runners can run tests in parallel
- Record video of the test run

You can also find me on the bevy discord as `@hajhawa`.

# Version table

| Bevy version | BITT version |
| ------------ | ------------ |
| 0.13         | 0.5          |
| 0.12         | 0.3          |

You may need to re-record your tests when upgrading to a new version of Bevy.
