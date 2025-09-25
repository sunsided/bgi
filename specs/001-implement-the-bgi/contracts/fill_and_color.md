# BGI Fill and Color API Contract

## floodfill

**Purpose**: Fill enclosed area with pattern starting from seed point
**Signature**: `fn floodfill(context: &mut GraphicsContext, x: i32, y: i32, boundary_color: Color) -> GraphResult`
**Parameters**: Seed point (x, y) coordinates and boundary color to stop fill
**Returns**: Error code (grOk or fill error)
**Side Effects**: Fills connected region with current fill pattern and color
**BGI Compatibility**: Direct mapping to `floodfill(x, y, border)` function

## setfillstyle

**Purpose**: Set fill pattern and color for area fill operations
**Signature**: `fn setfillstyle(context: &mut GraphicsContext, pattern: FillPattern, color: Color) -> GraphResult`
**Parameters**: Fill pattern type and fill color
**Returns**: Error code (grOk or pattern error)
**Side Effects**: Updates graphics context fill settings
**BGI Compatibility**: Direct mapping to `setfillstyle(pattern, color)` function

## getfillsettings

**Purpose**: Retrieve current fill pattern and color settings
**Signature**: `fn getfillsettings(context: &GraphicsContext) -> FillSettings`
**Parameters**: None (query function)
**Returns**: Current fill pattern and color values
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getfillsettings(fillinfo)` function

## setcolor

**Purpose**: Set current drawing color for lines and outlines
**Signature**: `fn setcolor(context: &mut GraphicsContext, color: Color) -> GraphResult`
**Parameters**: Color value to set as current drawing color
**Returns**: Error code (grOk or color error)
**Side Effects**: Updates graphics context drawing color
**BGI Compatibility**: Direct mapping to `setcolor(color)` function

## getcolor

**Purpose**: Get current drawing color
**Signature**: `fn getcolor(context: &GraphicsContext) -> Color`
**Parameters**: None (query function)
**Returns**: Current drawing color value
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getcolor()` function

## setbkcolor

**Purpose**: Set background color for graphics operations
**Signature**: `fn setbkcolor(context: &mut GraphicsContext, color: Color) -> GraphResult`
**Parameters**: Background color value
**Returns**: Error code (grOk or color error)
**Side Effects**: Updates background color, may clear screen
**BGI Compatibility**: Direct mapping to `setbkcolor(color)` function

## getbkcolor

**Purpose**: Get current background color
**Signature**: `fn getbkcolor(context: &GraphicsContext) -> Color`
**Parameters**: None (query function)
**Returns**: Current background color value
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getbkcolor()` function
