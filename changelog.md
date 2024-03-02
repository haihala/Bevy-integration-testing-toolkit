# 0.4 -> 0.5

`Asserter` was renamed to `TestWrangler` and gained a new `start` method. This is done automatically by default,
but in games where preparing the game state exceeds the startup schedule this could cause problems. If you want to
start playback / recording manually, set `PlaybackTestingOptions::manual_start` to `true` and call
`TestWrangler::start` when the game is ready. This function is idempotent so you can safely call it multiple
times.

Fixed several bugs, including not sending gamepad events on playback, which made certain setups not work.

# 0.4 or earlier

Look at the examples and try your best.
