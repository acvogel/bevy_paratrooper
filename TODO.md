# TODO

- [ ] Intro screen
  - [ ] PARATROOPER text animation
  - [ ] intro song
  - [ ] instructions? left/right/space. scoring.
  - [ ] advanced: difficulty to tune # of everything and speeds
- [ ] physics constants
  - [ ] sprite sizes
  - [ ] gravity/drag of paratroopers
  - [ ] base size
- [ ] dynamic assault system
  - [ ] move assault troops immediately upon landing
  - [ ] fix jump - a paratrooper sensor collider for jumping?
  - [ ] refactor to remove 
- [ ] endgame score UI. ideally with sprites of targets
- [ ] bombers
- [ ] energy weapon instead of weird bullets?
- [ ] levels
  - [ ] fixed # of planes
  - [ ] fixed # of paratroopers max from each plane
  - [ ] winning screen

# Notes
## Assault System

1. Re-simplify the states, don't use ground/air
2. make like AssaultState ? or just have the whole thing continuous.
3. "Ideal" design:

As paratroopers land, assign next available role.
Troopers begin acting roles immediately. 
Left/Right distinction? hrm. 
AssaultDirection component? or member vars like Base.
only need "side" for determining next available dude.

Behavior module: on_updates for each type, and then collisions.

Base: walk
Climber: walk, collision teleport? or jump, then "left" force above a given delta.
collide with wall: become static

Second Base: walk to climber, stop on collision

Sapper: Jump at Second Base, Jump a little at gun base.

## Version upgrade

* Collisions:
 - [ ] bullet <-> para
 - [ ] para <-> ground
* Gun
 - [X] rotation point
 - [X] angle boundary