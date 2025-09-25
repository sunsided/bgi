# BGI Mouse Extensions API Contract

## mousex

**Purpose**: Get current mouse cursor X coordinate
**Signature**: `fn mousex(context: &GraphicsContext) -> i32`
**Parameters**: None
**Returns**: Mouse X position in graphics coordinates
**Side Effects**: None, read-only mouse state access
**BGI Compatibility**: Extension function from SDL_bgi library

## mousey

**Purpose**: Get current mouse cursor Y coordinate
**Signature**: `fn mousey(context: &GraphicsContext) -> i32`
**Parameters**: None
**Returns**: Mouse Y position in graphics coordinates
**Side Effects**: None, read-only mouse state access
**BGI Compatibility**: Extension function from SDL_bgi library

## mouseclick

**Purpose**: Check for mouse button click event
**Signature**: `fn mouseclick(context: &mut GraphicsContext) -> MouseClick`
**Parameters**: None
**Returns**: Mouse click information (button, coordinates, timestamp)
**Side Effects**: May consume pending click events from queue
**BGI Compatibility**: Extension function from SDL_bgi library

## ismouseclick

**Purpose**: Test if mouse button click is pending
**Signature**: `fn ismouseclick(context: &GraphicsContext, button: MouseButton) -> bool`
**Parameters**: Mouse button to check (left, right, middle)
**Returns**: True if click event is pending for specified button
**Side Effects**: None, read-only event queue check
**BGI Compatibility**: Extension function from SDL_bgi library

## getmouseclick

**Purpose**: Get and consume mouse click event information
**Signature**: `fn getmouseclick(context: &mut GraphicsContext, button: MouseButton) -> Option<(i32, i32)>`
**Parameters**: Mouse button to get click for
**Returns**: Click coordinates if event exists, None otherwise
**Side Effects**: Consumes click event from queue
**BGI Compatibility**: Extension function from SDL_bgi library

## clearmouseclick

**Purpose**: Clear all pending mouse click events
**Signature**: `fn clearmouseclick(context: &mut GraphicsContext, button: MouseButton) -> GraphResult`
**Parameters**: Mouse button to clear events for
**Returns**: Error code (grOk or input error)
**Side Effects**: Removes all pending click events from queue
**BGI Compatibility**: Extension function from SDL_bgi library

## setmousequeuestatus

**Purpose**: Enable or disable mouse event queuing
**Signature**: `fn setmousequeuestatus(context: &mut GraphicsContext, status: MouseQueueStatus) -> GraphResult`
**Parameters**: Queue status (enabled, disabled, or specific modes)
**Returns**: Error code (grOk or configuration error)
**Side Effects**: Changes mouse event handling behavior globally
**BGI Compatibility**: Extension function from SDL_bgi library

## getmousequeuestatus

**Purpose**: Get current mouse event queue status
**Signature**: `fn getmousequeuestatus(context: &GraphicsContext) -> MouseQueueStatus`
**Parameters**: None
**Returns**: Current mouse queue configuration
**Side Effects**: None, read-only configuration access
**BGI Compatibility**: Extension function from SDL_bgi library
