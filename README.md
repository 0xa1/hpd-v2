# HPD v2 — RMK + Vial

A wired, 60-key split keyboard with two RP2040-Zero controllers and no additional modules. The right half is the `central`, and the left half is the `peripheral`. This project is based on the [official RMK template](https://github.com/rmk-rs/rmk-template/tree/1b2d061fb7b54600d4395792814462d6a1cebbb8/rp2040_split) and pulls RMK from crates.io. You don't need to clone RMK or Ergohaven to build the firmware.

## Download and flash

Open [Actions → Firmware](https://github.com/0xa1/hpd-v2/actions/workflows/firmware.yml), select a successful run for `main` or the pull request you want, and download **hpd-v2-uf2** from **Artifacts**.

| Half | Firmware file |
| --- | --- |
| Right (central) | `hpd-v2-right-central.uf2` |
| Left (peripheral) | `hpd-v2-left-peripheral.uf2` |

1. Disconnect power from the keyboard and unplug the cable between the halves. While holding **BOOT** on the half you're flashing, connect the **controller's onboard USB-C port** to your computer. Release **BOOT** when the `RPI-RP2` drive appears.
2. Copy the UF2 for that half to the drive. Repeat for the other half using the other UF2 from **the same archive**.
3. Disconnect the cable to your computer, reconnect the halves, then connect the **right** controller's onboard USB-C port to your computer.

When reflashing, you can use Bootmagic instead of the controller button: hold the **top-row key farthest from the split** on the half you're flashing while connecting that half to your computer. If the firmware doesn't start, use the physical **BOOT** button. You may need to open the case to reach it. This firmware doesn't use the double-press of **RESET** from Ergohaven's instructions.

## Wiring

The wiring is based on the [HPD handwiring guide](https://github.com/ergohaven/hpd/blob/main/handwired/handwired.md). Both halves use the same pins:

| Matrix | GPIO pins in RMK index order |
| --- | --- |
| Rows, top to bottom | GP6, GP5, GP4, GP3, GP2 |
| Columns, from the outer edge toward the split on the main rows | GP7, GP12, GP13, GP14, GP15, GP28 |

Use `COL2ROW` diode orientation, with the stripe on the row side. For the two inner thumb keys on each half, follow **the diagram in step 3**: the innermost key goes to GP12, and the key next to it goes to GP7. The final diagram in step 5 swaps them. You don't need the GP29 resistors for handedness detection.

**The side USB-C ports are only for connecting the halves. Do not connect them to a computer.** They carry power and UART0, not USB. The tables and photos in step 4 assign D+ and D− differently, so don't mix the two wiring schemes. Before applying power, check continuity through the cable: GP0 (TX) on each half must connect to GP1 (RX) on the other half, and 5 V and GND must connect to their matching pins. Do not connect both halves to a computer at the same time while they are linked together.

## Keymap and Vial

- [`keyboard.toml`](keyboard.toml) defines the wiring and factory keymap. The ported keymap is built in: four configured layers and twelve transparent layers. The Ctrl↔GUI toggle and five QMK keycodes for recording and playing back macros from the keyboard are disabled. You can still use regular Vial macros.
- [`vial.json`](vial.json) describes the physical key positions for Vial. If the matrix or physical layout changes, regenerate it from `keyboard.toml` with `rmkit layout convert --to-vial keyboard.toml`. Preserve the name `HPD v2`, the USB VID/PID, and `lighting: none`.
- [`hpd-v2.vil`](hpd-v2.vil) is a backup profile you can restore via **File → Load saved layout** in [Vial](https://get.vial.today/). You don't need to import it after flashing a fresh keyboard. Copy changes into `keyboard.toml` only if they should become factory defaults.

Vial changes survive a normal power cycle. Save your profile before reflashing: [a clean RMK build can reset your settings](https://rmk.rs/docs/features/storage#storage-is-cleared-when-rmk-is-rebuilt) even if the RMK version hasn't changed. This includes builds from GitHub Actions.

If key positions in the matrix change, update `hpd-v2.vil` and make sure it imports correctly on the new firmware. Don't import a personal `.vil` profile made for the old matrix as-is: Vial stores assignments by row and column number, not by physical position. Reapply your changes to the new layout in Vial, test the affected keys, and save a new `.vil` file.

When Vial prompts you to unlock the keyboard, hold the **keys at both ends of the right half's top row**. In **Matrix Tester**, test all 60 keys individually, paying particular attention to the four inner thumb keys. Then check tap and hold behavior for Space and Enter, layer 3 access with the two layer keys pressed in either order, media keys, and one-shot modifiers. After importing a profile, reconnect the keyboard and verify that the keymap was saved.

## Build and update

To build locally with `rustup` installed:

```sh
cargo install --locked cargo-make@0.37.24 flip-link@0.1.12 cargo-binutils@0.4.0 cargo-hex-to-uf2@0.1.2
cargo make uf2 --release
```

Dependabot checks for new releases of the official `rmk` crate every day. GitHub Actions builds both UF2 files using `Cargo.lock` for pull requests and `main`. Download the artifact from the pull request, test both halves on the keyboard, and only then merge the update manually.

**One-time setup after the first successful run:** In GitHub, go to **Settings → Rules → Rulesets** and create an active ruleset for `main` with no bypass actors. Enable **Require a pull request before merging** and **Require status checks to pass**. Select the **Build both HPD v2 UF2 files** check from GitHub Actions and require pull request branches to be up to date with `main` before merging. If you work alone, don't require approval from another person. Leave auto-merge disabled.
