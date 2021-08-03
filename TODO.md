# To-do:

[X] Implement a `Sampler`.
[X] Write `Film` to image.
[X] Start replacing old render function with new one that uses `Sampler`. Make sure demo still works.
[X] Parallelize render to reduce render times.
[X] Use Stanford teapot model.
[X] Light sources.
    [X] Point light.
[ ] Start replacing old `RayTracer` with limited version of `Whitted`.
    [X] Textures.
        [X] Constant texture.
    [ ] Implement some working BSDFs.
        [X] Perfect specular Fresnel objects.
        [X] Perfect diffuse Lambertian reflection.
        [X] Microfacet model.
    [ ] Minimal material library.
        [ ] Matte material.
        [ ] Plastic material.
[ ] Finish fleshing out `Whitted`.
    [ ] Visibility tester.
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