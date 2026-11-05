# Jade AE2 Plugin for Horizon

An unofficial port of the Jade plugin inside [Applied Energistics 2](https://github.com/AppliedEnergistics/Applied-Energistics-2)
15.4.12 to [Horizon](https://github.com/horizon-rust/horizon): network state, smart cable channels, P2P links, charge,
crafting and stored power in Jade, and naming the facade or cable under the crosshair.

**Needs both** [AE2 for Horizon](../../../ae2-horizon) and [Jade for Horizon](../../../jade-horizon) in the same build.

## What differs from the original

Rewritten in Rust as a small bridge between the two ports.

### Missing or not the same

No missing feature is known. Its lines cover what the AE2 port has, so they share that port's limits.

## Build order

This bundle calls into `mixins/ae2` and `mixins/jade` directly, so it won't compile without both in the same folder.

## Build

This is a [Horizon](https://github.com/horizon-rust/horizon) native mod bundle: Rust mixins compiled into the game.
It was built and tested on Horizon commit `6475f62` (Minecraft 26.3, native hooks API 359), as `native-6475f62-d72af06acdd9`.
Native mods are unsandboxed code that runs with your rights, so read what you build.

Put each bundle's `mixins/<id>` folder in one folder, then:

```bash
horizon-launcher native-build "path/to/that/folder" --trust-native-code
```

## Status

An unofficial port, not made or endorsed by the original authors. Please report problems here, not to them.
It was tested on Windows, in single player and on LAN with one guest. On a dedicated server its lines were checked on an earlier build,
not this one. Other platforms were not tested.
A Horizon update can break it until it is rebuilt: it is built for the one Horizon commit named under Build.

## Credits and licence

Applied Energistics 2, by AlgorithmX2 et al and TeamAppliedEnergistics. This port is a modified version of its plugin,
under the [GNU LGPL 3.0](LICENSE) (with [GPL 3.0](COPYING.GPL-3.0)). Jade is by Snownee.
