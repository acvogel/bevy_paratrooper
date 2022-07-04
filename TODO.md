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
- [X] dynamic assault system
  - [X] move assault troops immediately upon landing
  - [X] fix jump - a paratrooper sensor collider for jumping?
  - [X] refactor to remove 
- [ ] endgame score UI. ideally with sprites of targets
- [ ] bombers
- [ ] energy weapon instead of weird bullets?
- [ ] levels
  - [ ] fixed # of planes
  - [ ] fixed # of paratroopers max from each plane
  - [ ] winning screen

# Notes

## Bombers

Planes just like the others. drops bombs. bombs collide gun is gg. drop ballistic? if so, how
do we decide the drop point. are all on point? what if they hit paratroopers?
or bombers come in a 2nd wave.

1. SURPRISE ATTACK
2. BOMBERS
3. COMBINED ARMS

## Version upgrade

* Collisions:
 - [X] bullet <-> para
 - [X] para <-> ground
 - [X] para <-> gun
-  [X] para <-> gun mount
* Gun
 - [X] rotation point
 - [X] angle boundary