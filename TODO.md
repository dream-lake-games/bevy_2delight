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
