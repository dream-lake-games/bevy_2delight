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
