# To-do:

[ ] Finish fleshing out `Whitted`.
[ ] More BSDFs and materials.
    [ ] Microfacet BSDF. 
    [ ] plastic material.
[ ] More lights.
[ ] Move on to new reflection, material, and lighting chapters.

# Backlog:

[X] Implement a good `Filter`.
[X] Implement a good `Sampler`.
[ ] Add perspective camera.

# Refactoring

[X] Create a `Tile` struct and a method for generating a `Vec<Tile>`.
[X] Refactor `Integrator`.
    [X] No `Integrator` struct. Just a `render` function.
    [X] Create an `RayTrace` type alias for the `incoming_radiance` function type.
    [X] `render` function should take a `Scene`, `Camera`, `Filter`, and `RayTrace` as an arg.
[X] Pull `Film` out of `Camera`.
[ ] Split `Sampler` trait into three separate traits:
    [ ] `Sampler` - Has `start_pixel` and `start_next_sample` methods.
    [ ] `CloneWithSeed` - Enables the sampler to be cloned with a given seed.
    [ ] `IncrementalSampler` - Generates a single 1D or 2D sample for the current sample vector at a time.
    [ ] `ArraySampler` - Generates an array of 1D or 2D samples for the current sample vector at a time.
[ ] Replace `BxdfType` flag with two enums, `ScatteringHemisphere` and `Scattering
[ ] Maybe allow objects to store their transformation matrices to eliminate the `'mtrx` lifetime.