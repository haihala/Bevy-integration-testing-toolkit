# Plan

## Steps

0. Prepare (write a plan and setup quality control tools like clippy in github actions)
1. Make a test game that can be used to test all or many of points of contention listed below
2. Build tooling to test various aspects of the test game
3. Write other support resources like documentation and workflows that demonstrate how to use the tool
4. Write paper
5. Graduate

## Points of contention

1. Asset loading
2. Networking
3. Time scale / low perf CI
4. Running tests off the main thread (bevy likes to run on the main thread)

## Test game design

Multiplayer 2d platformer where you jump around collecting stars for a minute.

### Crates

Input handling - Leafwing
Physics - Rapier
Multiplayer - Matchbox?

### Game design

A and D to move sideways

Space to jump, you can jump whenever making contact with anything,
in which case you jump up and away from the contact thing.
This includes other players.

## Testing gear design

### User interface
- Tests are setup with the builder pattern
	- Constructor takes in Unit under test (Bevy plugin or plugin group, one constructor for each)
		- The application code being tested, ideally with minimal or no modifications
	- Builder methods
        - `add_plugins` - For adding test gear
		- `set_timeout` - modifies the default 10s timeout
		- `mock_input`
			- Takes in a path to an asset
			- If asset doesn't exist, game is launched normally and the player's inputs are recorded until window is closed, store those to a file
			- If file exists, use recorded inputs
		- `add_checklist`
			- Takes in a `Vec<impl Into<String>>`
				- You can define custom enums as well as use static str
			- Adds a `Res<Checklist<T>> where T impl Into<String>`
			- Test resource can check off elements on the list
- In world testing resource
	- Inserted automatically before App starts
	- Methods
		- `pass` - Mark test as passed
        - `fail` - Marks a test as failed
		- `check` - Check a box on the `Res<Checklist>`
			- If `Res<Checklist>` doesn't exist or the checked item doesn't exist on the list, test fails with an error message
			- If that was the last item to be checked, the test is passed
			- Timeout is reset whenever a box is checked


### Behind the curtain
- Support systems
	- Added to the world on test setup
	- List of systems
        - Timeout - Sees when the last checkbox was filled, fails the test if it has been too long

### Wants and extension options
- Rigidity (Fuck Cypress)
- Video and screenshots when tests fail
- Able to run in headless CI
    - Potentially helpful:
		- https://bevyengine.org/news/bevy-0-12/#external-renderer-context may be interesting
		- Check out how bevy_openxr does it
		- https://bevyengine.org/news/bevy-0-12/#example-execution-in-ci executes some examples in the CI, not sure how, why and so on. Seems like they use VMs that can show windows.
- Parallelization
- Automatic retries
	- If a test passes on a retry, give a warning about flaking
	- Not sure if this is a good idea
