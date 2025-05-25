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
- [ ] Want dialogue system that's easy and expressive

## Okay things that I should maybe do later

### Layering/lighting

It still can only hit 30fps on my old computer :(. Oh well.

I'm not really sure what to do to help. I need to learn more about why it's slow and go from there. Most frames it's creating only a handful of meshes, idk really how to speed it up there.

I think I need to (a) figure out if this actually matters and (b) get better at profiling, and a better understanding of how it works so I can reason about this.

I feel like also, the realistic thing to do is just expose performance settings, and allow people to turn off lighting. Will probably also be relevant for some particle things.

### Particles

Okay so I did this for zenith way back.

Let me reflect on this a little bit. I think what happened was that the API was hard to use, the pixels didn't look good, and the code was hard to maintain. So for redeath so far I've just been using aesprite for particles. But I think this is bad, and also not fun because I can't get particles that interact with physics, or brightness, or reflexivity, etc. Also does become kind of hell to maintain

Kami particles were actually pretty cool. I think they added to the game a lot.

I need a really good underlying abstraction for a single particle. What defines it? How does it get created? How does it die? How does it move?

Oh shit this is good rain

```
// Simple rain shader for Shadertoy
void mainImage(out vec4 fragColor, in vec2 fragCoord)
{
    // Normalize coordinates
    vec2 uv = fragCoord / iResolution.xy;

    // Rain parameters (hardcoded for Shadertoy)
    float rain_amount = 150.0;        // Number of potential raindrops
    float near_rain_length = 0.1;     // Length of close raindrops
    float far_rain_length = 0.02;      // Length of distant raindrops
    float slant = -0.1;                // How much the rain slants

    float near_rain_width = 1.0;      // Width of close raindrops
    float far_rain_width = 0.5;       // Width of distant raindrops
    float near_rain_transparency = 1.0; // Opacity of close raindrops
    float far_rain_transparency = 0.5;  // Opacity of distant raindrops

    float base_rain_speed = -1.1;      // Base falling speed
    float additional_rain_speed = -0.6; // Additional random speed

    vec3 rain_color = vec3(0.6, 0.6, 0.9); // Light gray color

    // Calculate grid position
    float remainder = mod((uv.x - uv.y * slant) * rain_amount, 1.0) / rain_amount;
    float grid_x = (uv.x - uv.y * slant) - remainder;

    // Generate random value for this column
    float rn = fract(sin(grid_x * rain_amount) * 43758.5453);

    // Calculate raindrop properties based on random value
    float length = mix(far_rain_length, near_rain_length, rn);
    float width = mix(far_rain_width, near_rain_width, rn);
    float transparency = mix(far_rain_transparency, near_rain_transparency, rn);
    float speed = base_rain_speed + additional_rain_speed * rn;

    // Calculate raindrop visibility
    float y_pos = fract(uv.y + rn - iTime * speed);
    float is_raindrop = step(1.0 - length, y_pos) * step(remainder * rain_amount, width);

    // Background color (dark blue-gray)
    vec3 bg_color = vec3(0.1, 0.12, 0.15);

    // Mix with background
    fragColor = vec4(mix(bg_color, rain_color, is_raindrop * transparency), 1.0);
}
```

What should the API for this "ShaderMan" be?

- Data-driven ;)
- ^basically the type resolving the generic should be the data that gets passed to the shader
- ^Also should have as some const or method on it to supply the path of the shader
- Then like AnimMan need easy ways to specify other things that all will have, layer, size (which we want for this but we infer for animman)
  // Okay going to bed but spam notes;
  // This
- this thing should know what the loop time is
- slant fucks up a vertical boundary, might not be able to use (probably fine)
- repetition should be able to be easily baked in (idk what should be on the man vs the struct, i.e.e a constant derive vs on the anim man)
  - Probably air on side of derive just is defaults, but all lives on the struct (including loop time)
