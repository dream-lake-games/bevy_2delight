# What's left?

## LDTK

What's the right abstraction for ldtk?

LdtkDelight

- Proper state with:
    - Unloaded
    - Loading
    - Loaded
    - Unloading

use events to trigger. Only valid transitions the outside world can trigger are:
- Unloaded -> Loading
- Loaded -> Unloading
- Everything else should be handled internally

We add a plugin:
- For each layer
- For each entity
- For each intcell (can be multiple values, or a single value)

We maintain:
- Current level
- Bounds of current level
- SpawnedLidActive/Inactive

DOn't think I need physical lid in/active, can add later if I have a compelling reason
