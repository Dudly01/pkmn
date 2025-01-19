<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [Quickstart](#quickstart)
- [Limitations](#limitations)
- [Help and Troubleshooting](#help-and-troubleshooting)
  - [expected image with minimal size of 160x144, got 1x1](#expected-image-with-minimal-size-of-160x144-got-1x1)
  - [could not locate Game Boy screen](#could-not-locate-game-boy-screen)
  - [could not recognize screen layout](#could-not-recognize-screen-layout)
  - [could not determine XXX DV range: stat value not found in stat variation XXX](#could-not-determine-xxx-dv-range-stat-value-not-found-in-stat-variation-xxx)
  - [could not read XXX: could not read character #X: could not recognize character](#could-not-read-xxx-could-not-read-character-x-could-not-recognize-character)
- [Credits](#credits)

<!-- /code_chunk_output -->

# PKMN

PKMN is a DV calculator, move and evolution lister webapp for Pokémon RBY (Gen I) and GSC (Gen II). It reads directly from the Game Boy screen and processes everything in the browser.

**See the potential of the freshly caught Pokémon.**  
Spare effort by focusing only on the very best, without the hassle of DV calculators.

**Read the details and side effects of the moves.**  
Battle confidently without learning from trial-and-error.

**See the evolutions of your Pokémon, when and what moves they learn through leveling up.**  
Raise your partner without missing a crucial move. No need to keep several browser tabs open.

**Read directly from the Game Boy screen then process it in the browser.**  
No manual data entry. No installation. No data leaves the device.

*"Optimize the fun out of the game. Rob yourself from the joy of exploration."* - Anonymus

## Quickstart

1. Select the screen with the Pokémon summary.
1. Select to scan once or periodically.
1. Check the result of the scan.

## Limitations

The app was tested on desktop browsers.

As of now, only the English version of the games is supported. However, as the field positions are similar across the language variants, the scanner may have partial functionaly. For example, the DV calculation is known to work for Pokémon Gelbe Edition, but the moves are not recognized and the Pokémon names will be in English.

The app can scan Game Boy emulators, videos and screenshots, if the conditions are met. The Game Boy screen needs to be fully visible, to be clean and not blurry, to have the original 10:9 aspect ratio, and to have no white borders. Photos taken from monitors will not work.

The tool should be used for approximating the DVs of freshly caught Pokémon. Through battles and certain items, Pokémon gain stat experience, which contributes to its stats. As the accumulated stat experience is hidden, the calculator assumes it to be zero. Therefore the more a Pokémon has trained, the less accurate its DV estimation will be.

DVs are calculated from the level, the base stat and the current stat value of a Pokémon. Finer details such as the shared DV value for Spc. Att and Spc. Def, the relation of HP DV to other DVs, or the effects of Gen II gender are not taken into account. To learn more, visit Bulbapedia.

## Help and Troubleshooting

Upon encountering errors, try the following steps:

- Choose a screen with the Game Boy screen on it.
- Have no white border around the Game Boy screen.
- Use the original 10:9 Game Boy screen aspect ratio.
- Have only one Game Boy visible, otherwise the biggest will be scanned.
- Show the Pokémon summary on the Game Boy.
- Use the original Game Boy resolution or one without distortions. Experiment with resizing the screen until successful scan.
- Avoid covering text with the cursor.
- Show snapshot or screen on the control panel to verify what is being scanned.

In case the issue remains, read further on for more specific instructions.

### expected image with minimal size of 160x144, got 1x1

This error appears, if the source window gets minimized. Open it up again and switch between the windows without minimizing them.

### could not locate Game Boy screen

The Game Boy screen was not found on the source screen. Make sure that it is indeed present on that. The Game Boy screen needs to be fully visible, without anything - including the cursor - covering the border. The screen needs to be in the original 10:9 aspect ratio. Furthermore, the screen should not have a white border.

### could not recognize screen layout

This error appears if the Game Boy screen has been (likely) found, but the screen it is showing is unrecognized. Make sure that the Game Boy is showing any of the summary screens, or switch to it otherwise. In addition to this, in rare occasions removing the cursor from the Game Boy screen can resolve the issue.

### could not determine XXX DV range: stat value not found in stat variation XXX

This error appears if a stat has an unexpected(ly high) value. The Pokémon likely gained stat experience through battling or through items like Protein, Iron, Carbos, etc.

### could not read XXX: could not read character #X: could not recognize character

A field could not be read from the screen. Make sure that nothing obstructs the visibility of the field, including the cursor. Make sure that the image is not blurry. Try resizing the Game Boy screen until this error goes away.

## Credits

This project would not have been possible without [Bulbapedia](https://bulbapedia.bulbagarden.net/), [Smogon](https://www.smogon.com/) and [Serebii.net](https://www.serebii.net/). The website aesthetics were borrowed from the [MDN Blog](https://developer.mozilla.org/en-US/blog/). Pokémon is a trademark of Nintendo.
