# BGI Palette and Color API Contract

## getdisplaycolor

**Purpose**: Get RGB values for color index in current palette
**Signature**: `fn getdisplaycolor(context: &GraphicsContext, color: u32) -> (u8, u8, u8)`
**Parameters**: Color index value
**Returns**: RGB components (red, green, blue) as tuple
**Side Effects**: None, read-only palette access
**BGI Compatibility**: Direct mapping to `getdisplaycolor(color)` function

## setpalette

**Purpose**: Set color value at specific palette index
**Signature**: `fn setpalette(context: &mut GraphicsContext, index: u32, color: u32) -> GraphResult`
**Parameters**: Palette index and new color value
**Returns**: Error code (grOk or palette error)
**Side Effects**: Modifies palette entry, may affect displayed graphics
**BGI Compatibility**: Direct mapping to `setpalette(colornum, color)` function

## setallpalette

**Purpose**: Replace entire palette with new color set
**Signature**: `fn setallpalette(context: &mut GraphicsContext, palette: &[u32]) -> GraphResult`
**Parameters**: Array of color values for all palette entries
**Returns**: Error code (grOk or palette error)
**Side Effects**: Replaces entire palette, affects all displayed graphics
**BGI Compatibility**: Direct mapping to `setallpalette(palette)` function

## getpalette

**Purpose**: Get current palette information structure
**Signature**: `fn getpalette(context: &GraphicsContext) -> PaletteInfo`
**Parameters**: None
**Returns**: Structure containing palette size and color array
**Side Effects**: None, read-only palette access
**BGI Compatibility**: Direct mapping to `getpalette(palette)` function

## getpalettesize

**Purpose**: Get number of colors in current palette
**Signature**: `fn getpalettesize(context: &GraphicsContext) -> u32`
**Parameters**: None
**Returns**: Number of palette entries available
**Side Effects**: None, read-only query
**BGI Compatibility**: Direct mapping to `getpalettesize()` function

## getdefaultpalette

**Purpose**: Get default palette for current graphics mode
**Signature**: `fn getdefaultpalette(context: &GraphicsContext) -> PaletteInfo`
**Parameters**: None
**Returns**: Default palette structure for current mode
**Side Effects**: None, returns static palette information
**BGI Compatibility**: Direct mapping to `getdefaultpalette()` function

## RGB

**Purpose**: Create 24-bit color value from RGB components
**Signature**: `fn rgb_color(red: u8, green: u8, blue: u8) -> u32`
**Parameters**: Red, green, blue components (0-255)
**Returns**: Packed RGB color value
**Side Effects**: None, pure color calculation
**BGI Compatibility**: Direct mapping to `RGB(r, g, b)` macro

## RED_VALUE

**Purpose**: Extract red component from RGB color value
**Signature**: `fn red_value(color: u32) -> u8`
**Parameters**: Packed RGB color value
**Returns**: Red component (0-255)
**Side Effects**: None, pure color extraction
**BGI Compatibility**: Direct mapping to `RED_VALUE(color)` macro

## GREEN_VALUE

**Purpose**: Extract green component from RGB color value
**Signature**: `fn green_value(color: u32) -> u8`
**Parameters**: Packed RGB color value
**Returns**: Green component (0-255)
**Side Effects**: None, pure color extraction
**BGI Compatibility**: Direct mapping to `GREEN_VALUE(color)` macro

## BLUE_VALUE

**Purpose**: Extract blue component from RGB color value
**Signature**: `fn blue_value(color: u32) -> u8`
**Parameters**: Packed RGB color value
**Returns**: Blue component (0-255)
**Side Effects**: None, pure color extraction
**BGI Compatibility**: Direct mapping to `BLUE_VALUE(color)` macro
