## Skate 3 - RK3326 Port

Native ARM64 port of **Skate 3** for the **M9 Pro / RK3326 / ArkOS4Clone**, built from the [skate-3-rust-engine](https://github.com/SK8-ENGINE/skate-3-rust-engine) decompilation and the [skate-3-vita](https://github.com/irolup/skate-3-vita) port.

### You need your own game data

This port contains **no game assets**. You must own an Xbox 360 copy of Skate 3 and convert it to `.pak` files on a Linux machine (Termux + proot-distro works on Android).

1. Extract your ISO with `extract-xiso`.
2. Run `prepare_assets.py` from the engine repo.
3. Run `vita-bake` to produce the `.pak` files.
4. Copy the `.pak` files into `skate3/assets/`.

Full guide: [skate-3-vita README](https://github.com/irolup/skate-3-vita#making-your-own-game-data).

### Controls

| Button | Action |
|---|---|
| Left stick | Skate / steer |
| A | Ollie |
| X | Grab |
| B | Push |
| Y | Trick modifier |
| Start | Pause |

### Performance notes (RK3326)

- Render resolution: **448x336**
- Shadows: **off**
- NPC draw distance: **10u**
- Traffic/pedestrian density: **25%**
- V-Sync: **off**

### Thanks

- **dumbad** for the years of Skate 3 reverse engineering.
- **acklavidian / SK8-ENGINE** for the Rust engine.
- **irolup** for the Vita port.
- **St0len-One** for the RK3326 WOTS2 port.

### License

GPL-3.0-only. Skate and Skate 3 belong to EA. This is an unofficial fan project.
