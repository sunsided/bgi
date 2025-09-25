# BGI Graphics Initialization API Contract

## initgraph
**Purpose**: Initialize graphics mode and create graphics context
**Signature**: `fn initgraph(mode: GraphicsMode) -> Result<GraphicsContext, BgiError>`
**Parameters**:
- `mode`: Graphics mode (CGA, EGA, VGA, custom resolution)
**Returns**:
- `Ok(GraphicsContext)` on successful initialization
- `Err(BgiError)` for invalid modes or system failures
**Side Effects**: Creates window, initializes graphics state, sets default values
**BGI Compatibility**: Maps to `initgraph(&gd, &gm, "")` with driver auto-detection

## closegraph
**Purpose**: Close graphics mode and cleanup resources
**Signature**: `fn closegraph(context: &mut GraphicsContext) -> Result<(), BgiError>`
**Parameters**:
- `context`: Mutable reference to graphics context
**Returns**:
- `Ok(())` on successful cleanup
- `Err(BgiError)` if context invalid or cleanup fails
**Side Effects**: Closes windows, releases resources, invalidates context
**BGI Compatibility**: Direct mapping to `closegraph()` with proper cleanup

## graphresult
**Purpose**: Get last graphics operation error code
**Signature**: `fn graphresult() -> GraphResult`
**Parameters**: None
**Returns**: Error code from last graphics operation (grOk, grInvalidDriver, etc.)
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `graphresult()` function

## grapherrormsg
**Purpose**: Get human-readable error message for error code
**Signature**: `fn grapherrormsg(error_code: i32) -> &'static str`
**Parameters**: BGI error code (grOk, grNoInitGraph, etc.)
**Returns**: Static string describing the error condition
**Side Effects**: None (pure function)
**BGI Compatibility**: Direct mapping to `grapherrormsg(errorcode)` function

## detectgraph
**Purpose**: Auto-detect best graphics driver and mode for system
**Signature**: `fn detectgraph(driver: &mut GraphicsDriver, mode: &mut GraphicsMode) -> GraphResult`
**Parameters**: Mutable references to receive detected driver and mode
**Returns**: Error code (grOk or detection error)
**Side Effects**: Updates driver and mode parameters with detected values
**BGI Compatibility**: Direct mapping to `detectgraph(&driver, &mode)` function

## getgraphmode
**Purpose**: Get current graphics mode information
**Signature**: `fn getgraphmode(context: &GraphicsContext) -> GraphicsMode`
**Parameters**:
- `context`: Reference to active graphics context
**Returns**: Current graphics mode details
**Side Effects**: None (query only)
**BGI Compatibility**: Maps to `getgraphmode()` with mode information

## setgraphmode
**Purpose**: Change graphics mode of existing context
**Signature**: `fn setgraphmode(context: &mut GraphicsContext, mode: GraphicsMode) -> GraphResult`
**Parameters**:
- `context`: Mutable reference to graphics context
- `mode`: New graphics mode to activate
**Returns**: Error code (grOk or specific error)
**Side Effects**: Changes resolution, resets graphics state, may clear screen
**BGI Compatibility**: Direct mapping to `setgraphmode(mode)` function
