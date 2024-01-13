# Plan

## Steps

0. Prepare (write a plan and setup quality control tools like clippy in github actions)
1. Make a test game
2. Build tooling to test various aspects of the test game
3. Write other support resources like documentation and workflows that demonstrate how to use the tool
4. Write paper
5. Graduate

## Test game design

2D platformer where you jump around collecting stars.

Arrows to move sideways. Space to jump, you can jump whenever, including midair

### Crates

Input handling - Leafwing
Physics - Rapier

## Testing gear design

### Playback testing

- Two modes
	- Record
		- Record inputs to a file
		- End when a condition is met
	- Playback
		- Read inputs from a file
		- Play them back
		- Take screenshots at the end
		- Check that the same conditions are met
- At the start of the test, test gear looks for a script
	- If it finds one, it does playback with that
	- If it doesn't, it records inputs to a file
		- Recording can be disabled with the read-only flag (for CI)

### Wants and extension options
- Rigidity (Fuck Cypress)
- Multi-part tests where it continues from a previous test case
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
