# BGI Viewport and Coordinate API Contract

## setviewport

**Purpose**: Define rectangular clipping region for drawing operations
**Signature**: `fn setviewport(context: &mut GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32, clip: bool) -> GraphResult`
**Parameters**: Top-left (x1, y1) and bottom-right (x2, y2) corners, clipping flag
**Returns**: Error code (grOk or viewport error)
**Side Effects**: Updates graphics context viewport settings, optionally enables clipping
**BGI Compatibility**: Direct mapping to `setviewport(left, top, right, bottom, clip)` function

## getviewsettings

**Purpose**: Retrieve current viewport configuration
**Signature**: `fn getviewsettings(context: &GraphicsContext) -> ViewportSettings`
**Parameters**: None (query function)
**Returns**: Current viewport coordinates and clipping state
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getviewsettings(viewport)` function

## clearviewport

**Purpose**: Clear current viewport area with background color
**Signature**: `fn clearviewport(context: &mut GraphicsContext) -> GraphResult`
**Parameters**: None (operates on current viewport)
**Returns**: Error code (grOk or clear error)
**Side Effects**: Fills viewport area with background color
**BGI Compatibility**: Direct mapping to `clearviewport()` function

## moveto

**Purpose**: Set current position (CP) for relative drawing operations
**Signature**: `fn moveto(context: &mut GraphicsContext, x: i32, y: i32) -> GraphResult`
**Parameters**: New current position coordinates (x, y)
**Returns**: Error code (grOk or coordinate error)
**Side Effects**: Updates graphics context current position
**BGI Compatibility**: Direct mapping to `moveto(x, y)` function

## moverel

**Purpose**: Move current position relative to current location
**Signature**: `fn moverel(context: &mut GraphicsContext, dx: i32, dy: i32) -> GraphResult`
**Parameters**: Relative offset (dx, dy) from current position
**Returns**: Error code (grOk or coordinate error)
**Side Effects**: Updates graphics context current position
**BGI Compatibility**: Direct mapping to `moverel(dx, dy)` function

## getx

**Purpose**: Get current position X coordinate
**Signature**: `fn getx(context: &GraphicsContext) -> i32`
**Parameters**: None (query function)
**Returns**: Current X position coordinate
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getx()` function

## gety

**Purpose**: Get current position Y coordinate
**Signature**: `fn gety(context: &GraphicsContext) -> i32`
**Parameters**: None (query function)
**Returns**: Current Y position coordinate
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `gety()` function

## getmaxx

**Purpose**: Get maximum X coordinate for current graphics mode
**Signature**: `fn getmaxx(context: &GraphicsContext) -> i32`
**Parameters**: None (query function)
**Returns**: Maximum X coordinate (width - 1)
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getmaxx()` function

## getmaxy

**Purpose**: Get maximum Y coordinate for current graphics mode
**Signature**: `fn getmaxy(context: &GraphicsContext) -> i32`
**Parameters**: None (query function)
**Returns**: Maximum Y coordinate (height - 1)
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getmaxy()` function
