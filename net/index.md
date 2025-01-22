<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [Quickstart](#quickstart)
- [Details](#details)
- [Troubleshooting](#troubleshooting)
- [Credits](#credits)

<!-- /code_chunk_output -->

# PKMN

PKMN is an app for Pokémon RBY and GSC. It calculates DVs and shows learnsets, evolutions and the details of known moves. No need to insert data into calculators or search online: PKMN shows you the info for the Pokémon you have selected in the game. It reads directly from the screen and processes everything in the browser; no data leaves your computer! 

## Quickstart

1. Open the summary screen (STATS) of a Pokémon in the game.
1. Click "Select screen" to select the window or screen that shows the game.
1. Click "Scan once" to scan the screen once and "Start scanning" to scan it continuosly.
1. See the results of the scan.

## Details

The app is intended for the English version of Pokémon RBY and GSC. However, it may function partially for other language variants as well. For example, the DV calculation is known to work with Pokémon Gelbe Edition.

PKMN can scan emulators, screenshots and videos -- if the conditions are met. The game screen needs to be fully visible, to have the original 10:9 aspect ratio, to have no direct white borders and to be not blurry or distorted. The Super Game Boy borders need to be disabled. Scanning is not expected to work with photos taken with a camera.

The DV calculation works for recently caught Pokémon. Through battling and consuming certain items, Pokémon gain stat experience that contributes to their total stat values. As this experience is hidden, the calculator assumes it to be 0. Bulbapedia describes [stat experience](https://bulbapedia.bulbagarden.net/wiki/Effort_values#Stat_experience) in more details.

DVs are calculated from the level, the base stat and the current stat value of a Pokémon. Finer details, such as the shared DV value for Spc. Att and Spc. Def, the effects of Gen II gender and the connection between the HP DV and other DVs are not yet taken into account. To learn more, visit [Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/Individual_values#Generation_I_and_II).

The app was tested with Firefox on PC.

## Troubleshooting

When encountering issues, following these steps can generally solve the majority of them:

- Refresh the page.
- Select the screen or window that shows the game.
- Make sure that there is no white border directly around the Game Boy screen.
- Disable the Super Game Boy borders.
- Use the original 10:9 aspect ratio for the game.
- Make sure that the screen is fully visible. Avoid covering the game with the cursor.
- Experiment with resizing the screen, especially if not using the original resolution.
- Make sure that the game shows the Pokémon summary (stats) screen.
- When multiple games are visible, the one with the bigger screen will be scanned.
- Click "Show snapshot" to see the image that was scanned last.
- Click "Show screen" to show the window or screen that the app receives as input.

With the help of the error messages, this process may be simplified.

**expected image with minimal size of 160x144, got 1x1**:  
This error appears if the source window is minimized.

**could not locate Game Boy screen**:  
The Game Boy screen was not found on the shared window or screen. Make sure it is indeed there. The game needs to be fully visible (mind the cursor), in the original 10:9 aspect ratio and have no white borders around. The Super Game Boy borders need to be disabled.

**could not recognize screen layout**:  
This error indicates that the Game Boy screen was found (likely correctly) and the game screen is not recognized. First, make sure that the game shows a Pokémon summary (stats) screen. Having the cursor on the Game Boy screen can also be a cause.

**could not determine XXX DV range: stat value not found in stat variation XXX**:  
This error appears if a stat has an unexpected(ly high) value. The Pokémon likely gained stat experience through battling or consuming certain items. The DV can not be calculated just from the visible stats anymore.

**could not read XXX: could not read character #X: could not recognize character**:  
A specific field could not be read because a character (letter) is not recognized. The game should be fully visible and not even the cursor should covers the texts. Make sure the image is not blurry and try resizing the game screen until the error goes away.

If in doubt, use the "Show snapshot" button to to see the image that was scanned last, and the "Show screen" to see the window or screen that the app receives as input.

## Credits

This project would not have been possible without [Bulbapedia](https://bulbapedia.bulbagarden.net/), [Smogon](https://www.smogon.com/) and [Serebii.net](https://www.serebii.net/). The website aesthetics were borrowed from the [MDN Blog](https://developer.mozilla.org/en-US/blog/). Pokémon is a trademark of Nintendo.
