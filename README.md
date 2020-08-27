# Golf Peaks

A solver for levels from [Golf Peaks](https://afterburn.itch.io/golf-peaks/) from [Afterburn Games](http://afterburn.games/).

![A video game puzzle where the player uses various moves to get a golf ball into the hole](./screenshot.png)

## Usage

Tested against the current macOS release (`v3.02`), though cards/levels may differ slightly between platforms/releases.

You can use my hand-transcribed level files if you'd like to see solutions for the first few worlds.

```sh
cargo run < old-levels/01-01.level.txt
```

Later on I managed to get in touch with the game creators, who sent me the source level files. Since I don't own these, I'm not including them here. If you'd like a copy, I'd suggest emailing them yourself.

You might find some subtle differences between the source levels and the levels in your game, so you'll need to edit those manually.

<!-- TODO create a patch -->

<details>
<summary><em>Differences between source level files and macOS release 3.02</em></summary>
<ul>
  <li><code>conveyor_hard1.asset</code> - Cards in different order</li>
  <li><code>extra_4.asset</code> - Cards in different order</li>
  <li><code>portal_1.asset</code> - Cards in a different order</li>
  <li><code>seven_z.asset</code> - Cards in a different order</li>
  <li><code>ten_2.asset</code> - Contains two ball tiles, remove the second</li>
  <li><code>ten_x1.asset</code> - Cards in a different order</li>
</ul>
</details>

```sh
tar -xf gp_levels.zip
cat gp_levels/roll_1.asset | python3 parse.py | cargo run
```

You can also see a full run of the game in action. It uses AppleScript to execute key presses, so you'll need to be on macOS. You'll also need to give your terminal permission to control your computer.

```sh
# Check that your terminal has permission to control your computer
# System Preferences > Settings & Privacy > Privacy > Accessibility
osascript -e 'tell application "System Events" to key code 36'

$SHELL full-run.sh
```

## Notes

### General Movement

There are three types of moves available.

- Rolling along the ground
- Chipping into the air
- Chipping into the air, then rolling after landing

Each move will travel a specific distance (through the air or rolling along the ground) before stopping.

A level is passed when the ball ends up in the hole. This might be because the ball stopped on the hole's tile, or because it was airborne and landed on the hole's tile.

### Walls

If a ball is rolling along the ground and it runs into a tile higher than it's current tile, it will bounce back in the oppositve direction. Airborne balls will clear all walls.

### Corners

Corners will reflect a rolling ball in a new direction. If the ball hits the back of a corner, it will bounce back in the opposite direction like a wall.

_For example, a ball heading south will bounce back if it run into a tile with a corner on its north side (whether NE/NW)._

### Slopes

Balls that land on a slope will begin rolling in the direction of the slope. When a ball is rolling down a slope, it will still lose energy as if it was rolling along regular ground. If a ball rolling down a slope runs out of energy, it will keep rolling until it reaches the bottom of the slope.

The only exception to this slope behaviour is if the ball is rolling along the ground and it begins heading directly uphill. If it runs out of energy while still going uphill, it will stop and turn around, rolling back to the bottom of the slope.

### Sand Traps

Balls that land on sand traps will lose all momentum. The only way for a ball to leave a trap is with an airborne move (it is a waste to use a rolling move).

### Quicksand

Quicksand behaves much like regular ground, except the level is failed if the balls stops moving on a quicksand tile.

### Water Hazards

Balls that land in water will be placed back on the last patch of _stable_ ground they rolled across. This might be where the original putt took place, or it might have been somewhere along the journey. Airborne balls that land in water will be set back to where they originated from.

### Springs

Springs launch rolling balls into the air, converting their rolling energy into airborne energy.

Note that a ball is only launched airborne when it _enters_ the tile, not if it stops on the spring and is moved again after that. If it rebounds off a wall though it will be launched, since it has "re-entered" the current tile.

### Portals

Portals come in pairs and behave similar to holes. A ball will be teleported to the corresponding exit portal if it either stops on the entry portal or lands in it after being airborne. If an airborne ball landing in a portal has remaining rolling energy, it will continue travelling after leaving the exit portal.

### Conveyor Belts

Conveyor belts are similar to slopes, except that the ball will only begin following the conveyor once it runs out of energy and stops.

### Ice

A ball that stops moving while on ice will continue rolling in its current direction until it either hits a wall and stops where it is, or leaves the ice.
