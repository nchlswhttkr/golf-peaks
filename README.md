# Golf Peaks

A solver for levels from [Golf Peaks](https://afterburn.itch.io/golf-peaks/).

---

### General Movement

There are three types of puts available.

- Rolling along the ground
- Chipping into the air
- Chipping into the air, then rolling after landing

Each put will travel a specific number of tiles (whether on the ground, airborne, or both) before stopping.

Passing a level involves finding the series of puts to land the ball in the hole. This means the ball must either stop on the hole's tile, or land in it after being airborne.

#### Walls

If a ball runs into a tile higher than its current tile, it will bounce back in the opposite direction. Airborne balls will clear the wall and continue along their existing path.

#### Corners

Corners either reflect the ball in a new direction or bounces it back, depending the orientation of the corner and the direction from which the ball approaches it.

_For example, incoming balls from the south or east will be redirected east and south respectively by a northwestern-facing corner. Meanwhile, incoming balls from the north or west will be reflected backwards._

---

### Terrain

Different terrains introduce new rules for how a ball will roll.

#### Slopes

Balls that land on a slope will be almost always begin following the direction of the slope. While rolling down a slope, the ball will still lose energy as if it were rolling along normal ground. If the ball stops while on a slope, it will continue rolling until it leaves the slope.

The only situation where a ball will not follow this rule is if it is rolling along the ground and begins rolling up the slope, in which case it will keep rolling upwards until it either stops or leaves the slope.

#### Sand Traps

Balls that land on sand traps will lose all momentum. The only way for a ball to leave rough terrain is with a putt that sends that ball airborne.

#### Quicksand

Quicksand behaves identically to regular ground, except that stopping on a quicksand tile constitutes failing a level.

#### Water Hazards

Balls that land in water will be placed back on the last patch of stable ground they rolled across. This might be where the original putt took place. Airborne balls that land in water will be set back to where they originated from.

#### Springs

Springs launch rolling balls into the air, converting their rolling energy into airborne energy.

Note that the balls is only launched airborne when it enters the tile, not if it stops on the spring and is putted again. If it rebounds off a wall though, it will be launched.

#### Portals

Portals come in pairs and behave similar to holes. A ball will be teleported to the corresponding exit portal if it either stops on an entry portals or lands in it after being airborne. If an airborne ball landing in a portal has remaining rolling energy, it will continue travelling after leaving the exit portal.

#### Conveyor Belts

Conveyor belts are similar to slopes, except that the ball will only begin following the conveyor once it runs out of energy and stops.

#### Ice

A ball that runs out of energy while on ice will continue rolling in its original direction until it either hits a wall and stops where it is, or leaves the ice.
