# BGI Text Rendering API Contract

## outtextxy

**Purpose**: Display text string at specified coordinates
**Signature**: `fn outtextxy(context: &mut GraphicsContext, x: i32, y: i32, text: &str) -> GraphResult`
**Parameters**: Display coordinates (x, y) and text string to render
**Returns**: Error code (grOk or text rendering error)
**Side Effects**: Renders text using current font, direction, and color
**BGI Compatibility**: Direct mapping to `outtextxy(x, y, textstring)` function

## outtext

**Purpose**: Display text at current position and advance cursor
**Signature**: `fn outtext(context: &mut GraphicsContext, text: &str) -> GraphResult`
**Parameters**: Text string to render at current position
**Returns**: Error code (grOk or text rendering error)
**Side Effects**: Renders text and updates current position (CP)
**BGI Compatibility**: Direct mapping to `outtext(textstring)` function

## settextstyle

**Purpose**: Configure font, direction, and character size for text output
**Signature**: `fn settextstyle(context: &mut GraphicsContext, font: Font, direction: TextDirection, char_size: u32) -> GraphResult`
**Parameters**: Font type, text direction (horizontal/vertical), character size multiplier
**Returns**: Error code (grOk or font error)
**Side Effects**: Updates graphics context text rendering settings
**BGI Compatibility**: Direct mapping to `settextstyle(font, direction, charsize)` function

## gettextsettings

**Purpose**: Retrieve current text rendering configuration
**Signature**: `fn gettextsettings(context: &GraphicsContext) -> TextSettings`
**Parameters**: None (query function)
**Returns**: Current font, direction, and size settings
**Side Effects**: None (query only)
**BGI Compatibility**: Direct mapping to `gettextsettings(texttypeinfo)` function

## textwidth

**Purpose**: Calculate pixel width of text string with current font
**Signature**: `fn textwidth(context: &GraphicsContext, text: &str) -> u32`
**Parameters**: Text string to measure
**Returns**: Width in pixels for the text string
**Side Effects**: None (measurement only)
**BGI Compatibility**: Direct mapping to `textwidth(textstring)` function

## textheight

**Purpose**: Calculate pixel height of text string with current font
**Signature**: `fn textheight(context: &GraphicsContext, text: &str) -> u32`
**Parameters**: Text string to measure
**Returns**: Height in pixels for the text string
**Side Effects**: None (measurement only)
**BGI Compatibility**: Direct mapping to `textheight(textstring)` function
