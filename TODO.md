# To-do:

[X] Replace camera with orthographic. Re-run demos.
[X] Compute ray differentials.
[ ] Film, Sampler, Filter
[ ] Replace ray-tracing logic in Scene with Witting Integrator. Re-run demos. 
[ ] Add perspective camera. Re-run demos.

# Refactoring

[ ] Create a `Tile` struct and a method for generating a `Vec<Tile>`.
[ ] Maybe create separate newtypes for `PixelBounds` and `SampleBounds`. To make the conversions clearer.
[ ] Refactor `Integrator`.
    [ ] No `Integrator` struct. Just a `render` function.
    [ ] Create an `RayTrace` type alias for the `incoming_radiance` function type.
    [ ] `render` function should take a `Scene`, `Camera`, `Filter`, and `RayTrace` as an arg.
[ ] Pull `Film` out of `Camera`.