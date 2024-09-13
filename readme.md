# Resources
- If there is only one global instance (singleton) of something, and it is standalone (not associated with other data), create a Resource.
```
#[derive(Resource)]
struct GameSettings {
    current_level: u32,
    difficulty: u32,
    max_time_seconds: u32,
}
```

- Access resources
```
fn my_system(
    // these will panic if the resources don't exist
    mut goals: ResMut<GoalsReached>,
    other: Res<MyOtherResource>,
    // use Option if a resource might not exist
    mut fancy: Option<ResMut<MyFancyResource>>,
) {
    if let Some(fancy) = &mut fancy {
        // TODO: do things with `fancy`
    }
    // TODO: do things with `goals` and `other`
}
```

## Create at runtime
```
fn my_setup(mut commands: Commands, /* ... */) {
    // add (or overwrite if existing) a resource, with the given value
    commands.insert_resource(GoalsReached { main_goal: false, bonus: 100 });
    // ensure resource exists (create it with its default value if necessary)
    commands.init_resource::<MyFancyResource>();
    // remove a resource (if it exists)
    commands.remove_resource::<MyOtherResource>();
}

```
# Systems
- Manage behaviours of entities
- Rust function (fn)
- Use Query in parameters to access entities
- Automatically run on CPU threads in parallel
```
fn spaceship_movement_controls(
    // Query for spaceship entity instead of everything with transform or velocity
    // Need to access keyboard controls
    // With argument - useful when want to query component but not the data. Just want to query entity with
    // Query - can put two arguments, each with tuples
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // single_mut - single entity associated with a component
    // will panic if not one copy
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }

    // Shift left or control left to rotate around local z axis
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // Rotate around the Y-axis.
    // Ignores the Z-axis rotation applied below.
    transform.rotate_y(rotation);

    // Rotate around the local Z-axis.
    // The rotation is relative to the current rotation!
    transform.rotate_local_z(roll);

    // Update the spaceship's velocity based on new direction.
    // Bevy considers forward in -Z direction
    // Can change this in Blender if use
    velocity.value = -transform.forward() * movement;
}
```

# Entities
- Unique ID stored in table
- Represents set of values for different components
- Can spawn and despawn entities
```
fn setup(mut commands: Commands) {
    // create a new entity
    commands.spawn((
        // Initialize all your components and bundles here
        Enemy,
        Health {
            hp: 100.0,
            extra: 25.0,
        },
        AiMode::Passive,
        // ...
    ));

    // If you want to get the Entity ID, just call `.id()` after spawn
    let my_entity = commands.spawn((/* ... */)).id();

    // destroy an entity, removing all data associated with it
    commands.entity(my_entity).despawn();
}
```

# Components
https://bevy-cheatbook.github.io/programming/ec.html#components
- Data associated with entities
- Struct or enum
```
#[derive(Component)]
struct Health {
    hp: f32,
    extra: f32,
}

#[derive(Component)]
enum AiMode {
    Passive,
    ChasingPlayer,
}
```
- Accessed from systems using Queries
- You can think of the query as the "specification" for the data you want to access. It gives you access to specific component values from entities that match the query's signature.
```
fn level_up_player(
    // get the relevant data. some components read-only, some mutable
    mut query_player: Query<(&PlayerName, &mut PlayerXp, &mut Health), With<Player>>,
) {
    // `single` assumes only one entity exists that matches the query
    let (name, mut xp, mut health) = query_player.single_mut();
    if xp.0 > 1000 {
        xp.0 = 0;
        health.hp = 100.0;
        health.extra += 25.0;
        info!("Player {} leveled up!", name.0);
    }
}

fn die(
    // `Entity` can be used to get the ID of things that match the query
    query_health: Query<(Entity, &Health)>,
    // we also need Commands, so we can despawn entities if we have to
    mut commands: Commands,
) {
    // we can have many such entities (enemies, player, whatever)
    // so we loop to check all of them
    for (entity_id, health) in query_health.iter() {
        if health.hp <= 0.0 {
            commands.entity(entity_id).despawn();
        }
    }
}
```

# Bundles
- Templates for creating entities
- MAke it easy to create entities with  common type of components
- By creating a bundle type, instead of adding your components one by one, you can make sure that you will never accidentally forget some important component on your entities. The Rust compiler will give an error if you do not set all the fields of a struct, thus helping you make sure your code is correct.

## Create
```
#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    marker: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    sprite: SpriteBundle,
}
```
## Spawn
```
commands.spawn(PlayerBundle {
    xp: PlayerXp(0),
    name: PlayerName("Player 1".into()),
    health: Health {
        hp: 100.0,
        extra: 0.0,
    },
    marker: Player,
    sprite: SpriteBundle {
        // TODO
        ..Default::default()
    },
});
```

## Alt - implement default then spawn
```
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            xp: PlayerXp(0),
            name: PlayerName("Player".into()),
            health: Health {
                hp: 100.0,
                extra: 0.0,
            },
            marker: Player,
            sprite: Default::default(),
        }
    }
}
```
- Spawn
```
commands.spawn(PlayerBundle {
    name: PlayerName("Player 1".into()),
    ..Default::default()
});
```

## Query
- Cant query a whole bundle, need to call individual components
```
fn my_system(query: Query<(&Transform, &Handle<Image>)>) {
  // ...
}
```

# Queries
https://bevy-cheatbook.github.io/programming/queries.html
- lets you access components of entities
- Use the Query system parameter, where you can specify the data you want to access, and optionally additional filters
- Use & for shared/readonly access and &mut for exclusive/mutable access
- To access all matching entities you will for loop:
```
fn check_zero_health(
    // access entities that have `Health` and `Transform` components
    // get read-only access to `Health` and mutable access to `Transform`
    // optional component: get access to `Player` if it exists
    mut query: Query<(&Health, &mut Transform, Option<&Player>)>,
) {
    // get all matching entities
    for (health, mut transform, player) in &mut query {
        eprintln!("Entity at {} has {} HP.", transform.translation, health.hp);

        // center if hp is zero
        if health.hp <= 0.0 {
            transform.translation = Vec3::ZERO;
        }

        if let Some(player) = player {
            // the current entity is the player!
            // do something special!
        }
    }
}
```

## Query Filters
Use With/Without to only get entities that have specific components.

```
fn debug_player_hp(
    // access the health (and optionally the PlayerName, if present), only for friendly players
    query: Query<(&Health, Option<&PlayerName>), (With<Player>, Without<Enemy>)>,
) {
    // get all matching entities
    for (health, name) in query.iter() {
        if let Some(name) = name {
            eprintln!("Player {} has {} HP.", name.0, health.hp);
        } else {
            eprintln!("Unknown player has {} HP.", health.hp);
        }
    }
}
```

# Commands
- Use Commands to spawn/despawn entities, add/remove components on existing entities, manage resources, from your systems.

```
fn spawn_things(
    mut commands: Commands,
) {
    // manage resources
    commands.insert_resource(MyResource::new());
    commands.remove_resource::<MyResource>();

    // create a new entity using `spawn`,
    // providing the data for the components it should have
    // (typically using a Bundle)
    commands.spawn(PlayerBundle {
        name: PlayerName("Henry".into()),
        xp: PlayerXp(1000),
        health: Health {
            hp: 100.0, extra: 20.0
        },
        _p: Player,
        sprite: Default::default(),
    });

    // you can use a tuple if you need additional components or bundles
    // (tuples of component and bundle types are considered bundles)
    // (note the extra parentheses)
    let my_entity_id = commands.spawn((
        // add some components
        ComponentA,
        ComponentB::default(),
        // add some bundles
        MyBundle::default(),
        TransformBundle::default(),
    )).id(); // get the Entity (id) by calling `.id()` at the end

    // add/remove components of an existing entity
    commands.entity(my_entity_id)
        .insert(ComponentC::default())
        .remove::<ComponentA>()
        .remove::<(ComponentB, MyBundle)>();

    // remove everything except the given components / bundles
    commands.entity(my_entity_id)
        .retain::<(TransformBundle, ComponentC)>();
}
```
# Sources
https://poly.pizza/bundle/Ultimate-Space-Kit-YWh743lqGX

Rover - https://poly.pizza/m/tzOLXetacM
