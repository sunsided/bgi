# BGI Input and Interaction API Contract

## getch

**Purpose**: Get single character from keyboard with wait
**Signature**: `fn getch(context: &mut GraphicsContext) -> Result<char, BgiError>`
**Parameters**: None (blocking input function)
**Returns**: Character code or input error
**Side Effects**: Blocks execution until key pressed
**BGI Compatibility**: Direct mapping to `getch()` function

## kbhit

**Purpose**: Check if keyboard input is available without blocking
**Signature**: `fn kbhit(context: &GraphicsContext) -> bool`
**Parameters**: None (non-blocking check)
**Returns**: True if key is available, false otherwise
**Side Effects**: None (polling only)
**BGI Compatibility**: Direct mapping to `kbhit()` function

## getmouseclick

**Purpose**: Get mouse click coordinates and button information
**Signature**: `fn getmouseclick(context: &mut GraphicsContext, button: MouseButton) -> Result<Option<(i32, i32)>, BgiError>`
**Parameters**: Mouse button to check for click
**Returns**: Click coordinates or None if no click
**Side Effects**: Consumes pending mouse click events
**BGI Compatibility**: SDL_bgi extension for mouse support

## mousex

**Purpose**: Get current mouse X coordinate
**Signature**: `fn mousex(context: &GraphicsContext) -> i32`
**Parameters**: None (position query)
**Returns**: Current mouse X position in graphics coordinates
**Side Effects**: None (query only)
**BGI Compatibility**: SDL_bgi extension for mouse support

## mousey

**Purpose**: Get current mouse Y coordinate
**Signature**: `fn mousey(context: &GraphicsContext) -> i32`
**Parameters**: None (position query)
**Returns**: Current mouse Y position in graphics coordinates
**Side Effects**: None (query only)
**BGI Compatibility**: SDL_bgi extension for mouse support

## delay

**Purpose**: Pause execution for specified milliseconds
**Signature**: `fn delay(milliseconds: u32)`
**Parameters**: Duration in milliseconds to pause
**Returns**: Nothing (void function)
**Side Effects**: Blocks execution for specified duration
**BGI Compatibility**: Direct mapping to `delay(milliseconds)` function
