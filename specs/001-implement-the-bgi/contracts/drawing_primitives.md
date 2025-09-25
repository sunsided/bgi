# BGI Drawing Primitives API Contract

## line
**Purpose**: Draw line between two points
**Signature**: `fn line(context: &mut GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult`
**Parameters**: Start point (x1, y1) and end point (x2, y2) coordinates
**Returns**: Error code (grOk or specific drawing error)
**Side Effects**: Updates pixel buffer, does not change current position
**BGI Compatibility**: Direct mapping to `line(x1, y1, x2, y2)` function

## circle
**Purpose**: Draw circle with specified center and radius
**Signature**: `fn circle(context: &mut GraphicsContext, x: i32, y: i32, radius: u32) -> GraphResult`
**Parameters**: Center coordinates (x, y) and radius in pixels
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws circle outline in current color and line style
**BGI Compatibility**: Direct mapping to `circle(x, y, radius)` function

## rectangle
**Purpose**: Draw rectangle outline between two corner points
**Signature**: `fn rectangle(context: &mut GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult`
**Parameters**: Top-left (x1, y1) and bottom-right (x2, y2) corners
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws rectangle outline in current color and line style
**BGI Compatibility**: Direct mapping to `rectangle(x1, y1, x2, y2)` function

## arc
**Purpose**: Draw circular arc segment
**Signature**: `fn arc(context: &mut GraphicsContext, x: i32, y: i32, start_angle: u32, end_angle: u32, radius: u32) -> GraphResult`
**Parameters**: Center (x, y), start/end angles in degrees, radius in pixels
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws arc segment in current color and line style
**BGI Compatibility**: Direct mapping to `arc(x, y, stangle, endangle, radius)` function

## putpixel
**Purpose**: Set individual pixel color
**Signature**: `fn putpixel(context: &mut GraphicsContext, x: i32, y: i32, color: Color) -> GraphResult`
**Parameters**: Pixel coordinates (x, y) and color value
**Returns**: Error code (grOk or coordinate error)
**Side Effects**: Sets single pixel in frame buffer
**BGI Compatibility**: Direct mapping to `putpixel(x, y, color)` function

## getpixel
**Purpose**: Get individual pixel color
**Signature**: `fn getpixel(context: &GraphicsContext, x: i32, y: i32) -> Result<Color, BgiError>`
**Parameters**: Pixel coordinates (x, y) to query
**Returns**: Pixel color value or coordinate error
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `getpixel(x, y)` function

## ellipse
**Purpose**: Draw elliptical arc segment
**Signature**: `fn ellipse(context: &mut GraphicsContext, x: i32, y: i32, start_angle: u32, end_angle: u32, x_radius: u32, y_radius: u32) -> GraphResult`
**Parameters**: Center (x, y), start/end angles, horizontal and vertical radii
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws elliptical arc in current color and line style
**BGI Compatibility**: Direct mapping to `ellipse(x, y, stangle, endangle, xradius, yradius)` function
