# What's left?

## LDTK

Basic version exists

- Proper state with:
    - Unloaded
    - Loading
    - Loaded
    - Unloading

use events to trigger. Only valid transitions the outside world can trigger are:
- Unloaded -> Loading
- Loaded -> Unloading
- Everything else should be handled internally

Each layer is registered with the app.

We add a plugin:
- For each entity
- For each intcell (can be multiple values, or a single value)

We maintain:
- Bounds of all spawned levels

Current level is still the actual underlying component

### TODO: Ldtk

Need to make spawned lid active/inactive

# Grrrrr

Shit feels bad

Needs fixing:

- [ ] Animation image flickering
    - Can solve with better system sets
- [x] Animations with odd sizes shouldn't be fucked
    - Just need to manually tweak position in anim man
- [x] Use a better fixed point impl
- [ ] Animation as part of build step so that it updates with files? I think? Or nonce and it's fine?
    - [ ] Actually this is probably what I want
        - [ ] Release - Basically what I have now. It's a const
        - [ ] Development - Have this be a resource. Have keyboard shortcut that will cause it to refresh


AHHH okay
- So shit's a little fucked because you were kinda throwing spaghetti at a wall
- You should start by reseting everything to sane, stable values even if it means that the effect is small, grey and bad
- Then you NEED to make sure HDR is working. It is key. Idk what you're going to do without it
- Then you want to try having multiple blur passes instead of just one
- Probably also want the brightness color to just be hardcoded instead of provided
- FUCK HDR nice
- Follow the actual guide for luminance to get brightness
- Grr also need to fix the lines from the cutout mat again
- And agree on a way to have brightness actually communicated? Maybe multiply by 8?
- Also probably want to have all the brightness layers render to the same camera.
- And have there be a default brightness of 0.0? Or do an ordered sampling? I guess I don't want to put a mesh on literally everything, so probably better to do ordered sampling
i.e. sample static brightness, if nothing then static pixel (to zero out if it's there), then detail brightness...
- And want tonemapping

## NEW TODO

- [x] Take another read through of all the layer code I just wrote, make sure it looks good.
- [x] Actually implement tonemapping (check that it's working, see how different stuff feels. 90% sure I want one with auto-correcting)
- [x] Don't do separate brightness culling per layer, combine, and then cull
    - [x] This should give: "I think I want the brightness threshold to be uniform across layers" for free
- [ ] Change the ownership model of light occlusion meshes, so we only recalculate when we actually need to
- [ ] Further reduce the number of meshes for light occlusion (I think one per light source would be great)
    - [ ] At least should be able to cut down by factor of 2 (or better maybe? Idk seems a little hard but possible)
        - [ ] Maybe closest point, then the two points near it? I think that works?
- [x] Want a "front detail" layer
- [ ] Want reflexivity to actually work
- [ ] Try to clean up the roots and code org of layer code to make it more readable/interpretable
- [x] Want removal of unneeded static hboxes
- [x] Want consolidation of static hboxes
- [ ] Want music, sound effects
- [ ] Want it to work in WASM
- [ ] Want sound effects, music
- [ ] Want particles
- [ ] Want save state

// Too tired to finish the thingy.
// I think the insight is this tho:
// - If I'm combining the brightness and reflexivity in this way (which I think I should) then I think I also need like a lit combine
// - Then the final brightness cull output needs to take the lit combine, the brightnes combine, the reflexivity combine, and then give a final culling
// I'm not sure this'll actually be faster performance wise but it'll certainly be more extensible.
