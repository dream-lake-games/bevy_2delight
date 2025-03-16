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
- [x] Change the ownership model of light occlusion meshes, so we only recalculate when we actually need to
- [x] Want a "front detail" layer
- [x] Want reflexivity to actually work
- [x] Collect observers to root
- [x] Want removal of unneeded static hboxes
- [x] Want consolidation of static hboxes
- [ ] Want to be able to set trigger hitboxes from aesprite... this is the dream...
- [ ] Want music, sound effects
- [ ] Want it to work in WASM
- [ ] Want sound effects, music
- [ ] Want particles
- [ ] Want save state

## Okay things that I should maybe do later

### Layering/lighting

It still can only hit 30fps on my old computer :(. Oh well.

I'm not really sure what to do to help. I need to learn more about why it's slow and go from there. Most frames it's creating only a handful of meshes, idk really how to speed it up there.

I think I need to (a) figure out if this actually matters and (b) get better at profiling, and a better understanding of how it works so I can reason about this.
