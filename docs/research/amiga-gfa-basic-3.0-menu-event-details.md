---
type: "Research Note"
title: "GFA BASIC 3.0 Amiga — menu and event details"
---

# GFA BASIC 3.0 Amiga — menu and event details

Source: *GFA BASIC 3.5 Interpreter*, Chapter 9, printed pp. 9-2–9-13. This is the 3.0 base-manual event/menu API; it is not 3.5 confirmation.

## Event polling model

The manual describes event handling as access to Intuition messages. `ON MENU ... GOSUB` installs a procedure to be called when a selected event occurs; the program polls for events when it executes `ON MENU` or `SLEEP`. `ON MENU t` waits for an event or for `t` Intui-ticks (tenths of a second). `SLEEP` is described as equivalent to `ON MENU 1`. Timer messages are generated only by active windows. (9-2, 9-9–9-10)

## BASIC pull-down menu representation

- `MENU m$()` presents a pull-down menu described by a string array. Each menu consists of a title, its entries, an empty-string terminator, and a further empty string terminating the whole menu. (9-4)
- An entry prefixed with `-` is displayed in light text and is not selectable. An entry prefixed with `!` introduces a subentry; further `!` entries nest under that subentry. (9-4)
- `MENU(x)` exposes the event message fields. The documented elements include menu index, IDCMP class, code, qualifier, address, mouse coordinates, event seconds/microseconds and window address. The OCR/table has gaps in the element numbering; preserve the printed page and verify exact indices before implementation. (9-5)
- `MENU KILL` removes the pull-down menu and disables the `ON MENU GOSUB` settings. `MENU x,y` changes the indexed entry; documented flags include inverse highlight, a box, disabled state and check mark. `MENU x,str$` changes its text, with excess characters ignored if longer than the original. `MENU KEY x,y` assigns/removes a keyboard shortcut invoked with the right Amiga key. (9-12)

## Event message classes

The manual maps `MENU(1)` event classes to window sizing, resize, refresh, mouse buttons/movement, gadget down/up, requester open/close, menu pick, window close, raw key, preferences, disk insert/remove, Workbench change, window activation/deactivation, delta mouse movement, vanilla ASCII key and timer tick. (9-6–9-8)

Notable details from the text:
- Mouse-button codes are reported through `MENU(2)`; the source gives distinct values for left/right press/release. (9-6)
- Menu-pick data is a bitfield; the manual gives expressions for menu number, entry and subentry. For BASIC menus, `MENU(0)` gives the selected string-array index. (9-7)
- Raw-key messages use scan codes; vanilla-key messages use the ASCII code under the active key map. (9-7–9-8)
- The timer event is described as one message every tenth of a second. (9-8)

## Procedure registration

| Form | Source-described role | Page |
|---|---|---:|
| `ON MENU GOSUB proc` | Handles BASIC menu selection; selected entry index is in `MENU(0)` | 9-10 |
| `ON MENU KEY GOSUB proc` | Handles keyboard input; key code is in `MENU(2)` | 9-10 |
| `ON MENU BUTTON GOSUB proc` | Handles mouse-button press/release; code is in `MENU(2)` | 9-10 |
| `ON MESSAGE GOSUB proc` | Handles any enabled message class | 9-10 |

Which messages are delivered depends on the IDCMP flags supplied when opening the window. The manual's example registers all four procedures and polls in a loop. (9-10–9-11)

## Version boundary

All details above are indexed in the 3.0 base chapter. The supplied 3.5 addendum does not independently restate the full menu/event subsystem. Keep these as inherited 3.0 candidates until confirmed by a named 3.5 source passage or interpreter observation. Do not infer Linux/SDL support from the Amiga Intuition event model.
