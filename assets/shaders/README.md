# The Bevy ECS Blueprint

## Global Context
> The overarching rules and data of the universe.

* **Resources:** Global Singleton Data (e.g., `Res<GameAssets>`, `Res<Time>`)
* **States:** Global Game Phases (e.g., `GameState::Playing`)
    * *Note: Used to restrict Systems via `.run_if()`*

---

## The Game World
> The actual things existing in your scene.

**Entities (The ID Numbers / The Glue)**

* **Entity 0v1 (The Player)**
    * Component: `Player` *(Marker Tag)*
    * Component: `Transform` *(Spatial Data)*
    * Component: `LinearVelocity` *(Avian2D Physics Data)*
    * Component: `PlayerAnimationState` *(Object-Level State Machine)*
* **Entity 0v2 (An Enemy)**
    * Component: `Enemy` *(Marker Tag)*
    * Component: `Transform` *(Spatial Data)*
    * Component: `Collider` *(Avian2D Physics Data)*

---

## The Engine Loop
> The Systems (Functions) that act on the World.

* **Startup Schedule:** Runs **ONCE** at boot.
    * *Examples:* `setup_camera`, trigger `AssetCollection`
* **FixedUpdate Schedule:** Runs at a guaranteed, steady cadence.
    * *Examples:* `apply_player_movement`, physics logic
* **Update Schedule:** Runs exactly **ONCE PER FRAME**.
    * Input Handling
    * Object State Transitions (Mutating Components)
    * Visual Reactions (Triggered instantly by `Changed<T>` filters)

---

## Communication
> How unrelated Systems talk to each other safely.

* **Broadcast:** System A (Collision) ➔ `EventWriter` ➔ **`EVENT: PlayerHit`**
* **Listen:**
    * **`EVENT: PlayerHit`** ➔ `EventReader` ➔ System B (Audio)
    * **`EVENT: PlayerHit`** ➔ `EventReader` ➔ System C (UI Health)
