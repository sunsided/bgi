# BGI Filled Shapes API Contract

## bar

**Purpose**: Draw filled rectangle (solid bar)
**Signature**: `fn bar(context: &mut GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32) -> GraphResult`
**Parameters**: Top-left (x1, y1) and bottom-right (x2, y2) corners
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Fills rectangle with current fill pattern and color
**BGI Compatibility**: Direct mapping to `bar(x1, y1, x2, y2)` function

## bar3d

**Purpose**: Draw 3D filled rectangle with border and depth
**Signature**: `fn bar3d(context: &mut GraphicsContext, x1: i32, y1: i32, x2: i32, y2: i32, depth: u32, top_flag: bool) -> GraphResult`
**Parameters**: Rectangle corners, 3D depth, and top visibility flag
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws 3D bar with fill pattern, border, and depth shading
**BGI Compatibility**: Direct mapping to `bar3d(x1, y1, x2, y2, depth, topflag)` function

## fillellipse

**Purpose**: Draw filled ellipse
**Signature**: `fn fillellipse(context: &mut GraphicsContext, x: i32, y: i32, x_radius: u32, y_radius: u32) -> GraphResult`
**Parameters**: Center coordinates (x, y) and horizontal/vertical radii
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Fills ellipse with current fill pattern and color
**BGI Compatibility**: Direct mapping to `fillellipse(x, y, xradius, yradius)` function

## fillpoly

**Purpose**: Draw filled polygon from point array
**Signature**: `fn fillpoly(context: &mut GraphicsContext, points: &[(i32, i32)]) -> GraphResult`
**Parameters**: Array of (x, y) coordinate pairs defining polygon vertices
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Fills closed polygon with current fill pattern and color
**BGI Compatibility**: Direct mapping to `fillpoly(numpoints, polypoints)` function

## drawpoly

**Purpose**: Draw polygon outline from point array
**Signature**: `fn drawpoly(context: &mut GraphicsContext, points: &[(i32, i32)]) -> GraphResult`
**Parameters**: Array of (x, y) coordinate pairs defining polygon vertices
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws polygon outline with current line style and color
**BGI Compatibility**: Direct mapping to `drawpoly(numpoints, polypoints)` function

## pieslice

**Purpose**: Draw filled pie slice (sector with lines to center)
**Signature**: `fn pieslice(context: &mut GraphicsContext, x: i32, y: i32, start_angle: u32, end_angle: u32, radius: u32) -> GraphResult`
**Parameters**: Center (x, y), start/end angles in degrees, radius in pixels
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws filled sector with lines to center using current fill pattern
**BGI Compatibility**: Direct mapping to `pieslice(x, y, stangle, endangle, radius)` function

## sector

**Purpose**: Draw filled elliptical sector
**Signature**: `fn sector(context: &mut GraphicsContext, x: i32, y: i32, start_angle: u32, end_angle: u32, x_radius: u32, y_radius: u32) -> GraphResult`
**Parameters**: Center (x, y), start/end angles, horizontal/vertical radii
**Returns**: Error code (grOk or drawing error)
**Side Effects**: Draws filled elliptical sector with current fill pattern
**BGI Compatibility**: Direct mapping to `sector(x, y, stangle, endangle, xradius, yradius)` function
