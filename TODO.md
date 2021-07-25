# Granular to-do:

[X] Implement a `Sampler`.
[X] Write `Film` to image.
[X] Start replacing old render function with new one that uses `Sampler`. Make sure demo still works.
[ ] Parallelize render to reduce render times.
[ ] Start replacing old `RayTracer` with limited version of `Whitted`. 
[ ] Finish fleshing out `Whitted`.
[ ] Move on to new reflection, material, and lighting chapters.

# Old to-do list:

[X] Replace camera with orthographic. Re-run demos.
[X] Compute ray differentials.
[ ] Film, Sampler, Filter
[ ] Replace ray-tracing logic in Scene with Witting Integrator. Re-run demos. 
[ ] Add perspective camera. Re-run demos.

# Refactoring

[X] Create a `Tile` struct and a method for generating a `Vec<Tile>`.
[X] Refactor `Integrator`.
    [X] No `Integrator` struct. Just a `render` function.
    [X] Create an `RayTrace` type alias for the `incoming_radiance` function type.
    [X] `render` function should take a `Scene`, `Camera`, `Filter`, and `RayTrace` as an arg.
[X] Pull `Film` out of `Camera`.