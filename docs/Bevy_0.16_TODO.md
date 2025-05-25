# Upgrade Plan

Here are all the things from the release notes that I need to incorporate into 2delight

## [ECS Relationships](https://bevyengine.org/news/bevy-0-16/#ecs-relationships)

## [Improved Spawn Api](https://bevyengine.org/news/bevy-0-16/#improved-spawn-api)

## [Unified Error Handling](https://bevyengine.org/news/bevy-0-16/#unified-ecs-error-handling)

## [Measure Transform Propogation Improvements](https://bevyengine.org/news/bevy-0-16/#faster-transform-propagation)

First step: remove any framerate limiting (I think)

## [Immutable Components](https://bevyengine.org/news/bevy-0-16/#immutable-components)

## [Entity Cloning](https://bevyengine.org/news/bevy-0-16/#entity-cloning)

I kinda forgot how I did my ldtk stuff. At one point I was fake cloning stuff. If that's still the case I probably want to try something similar.

## [Disabling Entities](https://bevyengine.org/news/bevy-0-16/#entity-disabling-default-query-filters)

Maybe will be worth it for animation stuff? Probably

Also for particles that are offscreen? Maybe? Idk tho bc we probably still want their lifespan to update, so maybe something more complex here? Idk
