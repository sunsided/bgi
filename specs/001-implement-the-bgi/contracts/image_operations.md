# BGI Image Operations API Contract

## getimage

**Purpose**: Capture rectangular screen region to buffer
**Signature**: `fn getimage(context: &GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<ImageBuffer, GraphResult>`
**Parameters**: Top-left (x1, y1) and bottom-right (x2, y2) corners
**Returns**: Captured image buffer or error code
**Side Effects**: Allocates buffer memory, copies screen pixels
**BGI Compatibility**: Direct mapping to `getimage(x1, y1, x2, y2, bitmap)` function

## putimage

**Purpose**: Draw image buffer to screen with merge mode
**Signature**: `fn putimage(context: &mut GraphicsContext, x: i32, y: i32, buffer: &ImageBuffer, mode: PutImageMode) -> GraphResult`
**Parameters**: Destination coordinates (x, y), image buffer, and merge mode
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Renders image to screen using specified merge operation
**BGI Compatibility**: Direct mapping to `putimage(x, y, bitmap, op)` function

## imagesize

**Purpose**: Calculate memory size needed for rectangular region
**Signature**: `fn imagesize(x1: i32, y1: i32, x2: i32, y2: i32) -> u32`
**Parameters**: Top-left (x1, y1) and bottom-right (x2, y2) corners
**Returns**: Memory size in bytes needed for image buffer
**Side Effects**: Pure calculation, no graphics state changes
**BGI Compatibility**: Direct mapping to `imagesize(x1, y1, x2, y2)` function

## loadimage

**Purpose**: Load image from file format (BMP, GIF, JPEG, PNG)
**Signature**: `fn loadimage(context: &mut GraphicsContext, filename: &str) -> Result<ImageBuffer, GraphResult>`
**Parameters**: File path string
**Returns**: Loaded image buffer or error code
**Side Effects**: File I/O, memory allocation for image data
**BGI Compatibility**: Extension function from SDL_bgi library

## saveimage

**Purpose**: Save current screen or buffer to file format
**Signature**: `fn saveimage(context: &GraphicsContext, filename: &str, buffer: Option<&ImageBuffer>) -> GraphResult`
**Parameters**: File path, optional buffer (None for full screen)
**Returns**: Error code (grOk or file/format error)
**Side Effects**: File I/O, creates or overwrites file
**BGI Compatibility**: Extension function from SDL_bgi library

## readimagefile

**Purpose**: Load and display image file at coordinates
**Signature**: `fn readimagefile(context: &mut GraphicsContext, filename: &str, x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult`
**Parameters**: File path, destination rectangle corners
**Returns**: Error code (grOk or file/drawing error)
**Side Effects**: File I/O, scales and renders image to specified region
**BGI Compatibility**: Direct mapping to `readimagefile(filename, x1, y1, x2, y2)` function

## writeimagefile

**Purpose**: Save rectangular screen region to image file
**Signature**: `fn writeimagefile(context: &GraphicsContext, filename: &str, x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult`
**Parameters**: File path, source rectangle corners
**Returns**: Error code (grOk or file/capture error)
**Side Effects**: File I/O, captures and saves screen region
**BGI Compatibility**: Direct mapping to `writeimagefile(filename, x1, y1, x2, y2)` function
