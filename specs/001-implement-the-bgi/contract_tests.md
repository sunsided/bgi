# BGI API Contract Tests

## Graphics Initialization Tests

```rust
#[cfg(test)]
mod graphics_init_tests {
    use super::*;

    #[test]
    fn test_initgraph_detect_driver() {
        let result = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "");
        assert!(result.is_ok());
        let mut ctx = result.unwrap();
        assert_eq!(graphresult(), GraphResult::grOk);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_initgraph_invalid_mode() {
        let result = initgraph(GraphicsDriver::VGA, GraphicsMode::VgaHi, "invalid_path");
        // Should handle gracefully and either succeed with fallback or return error
        match result {
            Ok(mut ctx) => closegraph(&mut ctx),
            Err(_) => assert_eq!(graphresult(), GraphResult::grInvalidDriver),
        }
    }

    #[test]
    fn test_closegraph_cleanup() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        closegraph(&mut ctx);
        // After closegraph, subsequent graphics operations should fail
        assert_eq!(graphresult(), GraphResult::grNoInitGraph);
    }

    #[test]
    fn test_getgraphmode() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let mode = getgraphmode(&ctx);
        assert!(matches!(mode, GraphicsMode::Default | GraphicsMode::VgaHi));
        closegraph(&mut ctx);
    }

    #[test]
    fn test_setgraphmode() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let result = setgraphmode(&mut ctx, GraphicsMode::VgaHi);
        assert!(result.is_ok());
        assert_eq!(getgraphmode(&ctx), GraphicsMode::VgaHi);
        closegraph(&mut ctx);
    }
}
```

## Drawing Primitives Tests

```rust
#[cfg(test)]
mod drawing_tests {
    use super::*;

    fn setup_graphics() -> GraphicsContext {
        initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap()
    }

    #[test]
    fn test_line_drawing() {
        let mut ctx = setup_graphics();
        let result = line(&mut ctx, 0, 0, 100, 100);
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }

    #[test]
    fn test_circle_drawing() {
        let mut ctx = setup_graphics();
        let result = circle(&mut ctx, 150, 150, 50);
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }

    #[test]
    fn test_rectangle_drawing() {
        let mut ctx = setup_graphics();
        let result = rectangle(&mut ctx, 50, 50, 200, 150);
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }

    #[test]
    fn test_putpixel_getpixel() {
        let mut ctx = setup_graphics();
        let color = Color::Red;
        putpixel(&mut ctx, 100, 100, color).unwrap();
        let retrieved_color = getpixel(&ctx, 100, 100).unwrap();
        assert_eq!(retrieved_color, color);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_arc_drawing() {
        let mut ctx = setup_graphics();
        let result = arc(&mut ctx, 200, 200, 0, 90, 75);
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }
}
```

## Color and Fill Tests

```rust
#[cfg(test)]
mod color_fill_tests {
    use super::*;

    #[test]
    fn test_setcolor_getcolor() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        setcolor(&mut ctx, Color::Blue).unwrap();
        assert_eq!(getcolor(&ctx), Color::Blue);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_setbkcolor_getbkcolor() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        setbkcolor(&mut ctx, Color::Green).unwrap();
        assert_eq!(getbkcolor(&ctx), Color::Green);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_setfillstyle_getfillsettings() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        setfillstyle(&mut ctx, FillPattern::Solid, Color::Yellow).unwrap();
        let settings = getfillsettings(&ctx);
        assert_eq!(settings.pattern, FillPattern::Solid);
        assert_eq!(settings.color, Color::Yellow);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_floodfill() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        // Draw a closed shape first
        setcolor(&mut ctx, Color::Red).unwrap();
        circle(&mut ctx, 150, 150, 50).unwrap();

        // Fill it
        setfillstyle(&mut ctx, FillPattern::Solid, Color::Blue).unwrap();
        let result = floodfill(&mut ctx, 150, 150, Color::Red);
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }
}
```

## Text Rendering Tests

```rust
#[cfg(test)]
mod text_tests {
    use super::*;

    #[test]
    fn test_outtextxy() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let result = outtextxy(&mut ctx, 50, 50, "Test text");
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }

    #[test]
    fn test_outtext_updates_position() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        moveto(&mut ctx, 100, 100).unwrap();
        let initial_x = getx(&ctx);

        outtext(&mut ctx, "Hello").unwrap();
        let final_x = getx(&ctx);

        assert!(final_x > initial_x); // Position should advance
        closegraph(&mut ctx);
    }

    #[test]
    fn test_settextstyle_gettextsettings() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        settextstyle(&mut ctx, Font::Default, TextDirection::Vertical, 2).unwrap();

        let settings = gettextsettings(&ctx);
        assert_eq!(settings.font, Font::Default);
        assert_eq!(settings.direction, TextDirection::Vertical);
        assert_eq!(settings.charsize, 2);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_text_dimensions() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let text = "Test string";
        let width = textwidth(&ctx, text);
        let height = textheight(&ctx, text);

        assert!(width > 0);
        assert!(height > 0);
        closegraph(&mut ctx);
    }
}
```

## Input and Interaction Tests

```rust
#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn test_kbhit_no_input() {
        let ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        // Should return false when no input available
        assert_eq!(kbhit(&ctx), false);
        let mut ctx = ctx;
        closegraph(&mut ctx);
    }

    #[test]
    fn test_mouse_position() {
        let ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let x = mousex(&ctx);
        let y = mousey(&ctx);

        // Mouse coordinates should be within screen bounds
        assert!(x >= 0 && x <= getmaxx(&ctx));
        assert!(y >= 0 && y <= getmaxy(&ctx));
        let mut ctx = ctx;
        closegraph(&mut ctx);
    }

    #[test]
    fn test_getmouseclick_no_click() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let result = getmouseclick(&mut ctx, MouseButton::Left);

        // Should return None when no click available
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_delay() {
        use std::time::Instant;
        let start = Instant::now();
        delay(100); // 100ms delay
        let elapsed = start.elapsed().as_millis();

        // Should delay approximately 100ms (allow some tolerance)
        assert!(elapsed >= 90 && elapsed <= 150);
    }
}
```

## Viewport and Coordinate Tests

```rust
#[cfg(test)]
mod viewport_tests {
    use super::*;

    #[test]
    fn test_setviewport_getviewsettings() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        setviewport(&mut ctx, 50, 50, 300, 200, true).unwrap();

        let settings = getviewsettings(&ctx);
        assert_eq!(settings.left, 50);
        assert_eq!(settings.top, 50);
        assert_eq!(settings.right, 300);
        assert_eq!(settings.bottom, 200);
        assert_eq!(settings.clip, true);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_clearviewport() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        setviewport(&mut ctx, 100, 100, 400, 300, true).unwrap();

        // Draw something in viewport
        line(&mut ctx, 0, 0, 100, 100).unwrap();

        // Clear should succeed
        let result = clearviewport(&mut ctx);
        assert!(result.is_ok());
        closegraph(&mut ctx);
    }

    #[test]
    fn test_moveto_getx_gety() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        moveto(&mut ctx, 150, 200).unwrap();

        assert_eq!(getx(&ctx), 150);
        assert_eq!(gety(&ctx), 200);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_moverel() {
        let mut ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        moveto(&mut ctx, 100, 100).unwrap();
        moverel(&mut ctx, 50, -25).unwrap();

        assert_eq!(getx(&ctx), 150);
        assert_eq!(gety(&ctx), 75);
        closegraph(&mut ctx);
    }

    #[test]
    fn test_getmaxx_getmaxy() {
        let ctx = initgraph(GraphicsDriver::Detect, GraphicsMode::Default, "").unwrap();
        let max_x = getmaxx(&ctx);
        let max_y = getmaxy(&ctx);

        // Should return positive values
        assert!(max_x > 0);
        assert!(max_y > 0);

        // Should be reasonable screen dimensions
        assert!(max_x >= 639); // Minimum VGA width - 1
        assert!(max_y >= 479); // Minimum VGA height - 1
        let mut ctx = ctx;
        closegraph(&mut ctx);
    }
}
```
