//! Interactive BGI demo with the minifb visual backend.
//!
//! Mouse-driven drawing:
//! - Move the mouse: a preview of the current shape follows the cursor.
//! - Left click: stamp the current shape at the cursor (stamps persist).
//! - Right click or 'c': cycle the drawing color.
//! - 's': cycle the shape (circle / square / triangle).
//! - Middle click or 'x': clear all stamps.
//! - ESC or 'q': quit.

use bgi::*;

/// A shape stamped onto the canvas by a click.
#[derive(Clone, Copy)]
struct Stamp {
    kind: usize,
    x: i32,
    y: i32,
    color: Color,
}

const SHAPE_NAMES: [&str; 3] = ["Circle", "Square", "Triangle"];
const STAMP_RADIUS: i32 = 24;

fn palette() -> [(&'static str, Color); 7] {
    [
        ("RED", Color::RED),
        ("GREEN", Color::GREEN),
        ("BLUE", Color::LIGHTBLUE),
        ("YELLOW", Color::YELLOW),
        ("CYAN", Color::CYAN),
        ("MAGENTA", Color::MAGENTA),
        ("WHITE", Color::WHITE),
    ]
}

/// Draw the animated backdrop: a set of orbiting circles linked by lines.
/// `frame` advances the animation; `(cx, cy)` is the screen center.
fn draw_background(frame: u32, cx: i32, cy: i32) {
    const ORBIT_COLORS: [Color; 5] = [
        Color::DARKGRAY,
        Color::LIGHTBLUE,
        Color::LIGHTGREEN,
        Color::LIGHTCYAN,
        Color::LIGHTMAGENTA,
    ];

    let t = frame as f32 * 0.06;
    let mut prev: Option<(i32, i32)> = None;

    for i in 0..5i32 {
        let speed = (i + 1) as f32;
        // Multiply by the orbit radius *before* casting (casting a unit-range
        // cos/sin to i32 first would truncate it to 0 and kill the motion).
        let rx = (40 + i * 18) as f32;
        let ry = (28 + i * 9) as f32;
        let ox = cx + ((t * speed).cos() * rx) as i32;
        let oy = cy + ((t * speed).sin() * ry) as i32;

        // Connecting line from the previous orbit body.
        if let Some((px, py)) = prev {
            setcolor(Color::DARKGRAY);
            line(px, py, ox, oy);
        }
        prev = Some((ox, oy));

        setcolor(ORBIT_COLORS[i as usize]);
        circle(ox, oy, 10 + i * 6);
    }
}

/// Draw a shape outline of the given kind centered at (x, y).
fn draw_shape(kind: usize, x: i32, y: i32, radius: i32) {
    match kind {
        // Circle
        0 => circle(x, y, radius),
        // Square
        1 => rectangle(x - radius, y - radius, x + radius, y + radius),
        // Triangle
        _ => {
            let (ax, ay) = (x, y - radius);
            let (bx, by) = (x - radius, y + radius);
            let (cx, cy) = (x + radius, y + radius);
            line(ax, ay, bx, by);
            line(bx, by, cx, cy);
            line(cx, cy, ax, ay);
        }
    }
}

fn main() {
    let mut driver = 9; // VGA
    let mut mode = 2; // VGAHI (640x480, 16 colors)

    initgraph(&mut driver, &mut mode, "");
    if graphresult() != GraphResult::Ok {
        eprintln!("Graphics initialization failed!");
        return;
    }

    // Batch mode + refresh() per frame keeps the window flicker-free.
    set_batch_mode(true);
    setbkcolor(Color::BLACK);

    let colors = palette();
    let mut color_idx = 0usize;
    let mut shape_idx = 0usize;
    let mut stamps: Vec<Stamp> = Vec::new();
    let mut frame = 0u32;

    println!("Interactive BGI Demo");
    println!("Left click: stamp shape | Right click / 'c': color | 's': shape");
    println!("Middle click / 'x': clear | ESC or 'q': quit");

    loop {
        let mx = mousex();
        let my = mousey();
        let current_color = colors[color_idx].1;

        // --- Handle mouse clicks (clear after handling so each physical
        //     click is processed exactly once). ---
        if ismouseclick(1) {
            stamps.push(Stamp {
                kind: shape_idx,
                x: mx,
                y: my,
                color: current_color,
            });
            clearmouseclick(1);
        }
        if ismouseclick(2) {
            color_idx = (color_idx + 1) % colors.len();
            clearmouseclick(2);
        }
        if ismouseclick(4) {
            stamps.clear();
            clearmouseclick(4);
        }

        // --- Handle keyboard ---
        if kbhit()
            && let Some(ch) = getch()
        {
            match ch {
                'q' | 'Q' => break,
                c if c as u8 == 27 => break, // ESC
                'c' | 'C' => color_idx = (color_idx + 1) % colors.len(),
                's' | 'S' => shape_idx = (shape_idx + 1) % SHAPE_NAMES.len(),
                'x' | 'X' => stamps.clear(),
                _ => {}
            }
        }

        // --- Render frame ---
        cleardevice();

        // Animated backdrop (always running, behind everything)
        draw_background(frame, getmaxx() / 2, getmaxy() / 2);

        // Persistent stamps
        for stamp in &stamps {
            setcolor(stamp.color);
            draw_shape(stamp.kind, stamp.x, stamp.y, STAMP_RADIUS);
        }

        // Live preview of the current shape at the cursor
        setcolor(current_color);
        draw_shape(shape_idx, mx, my, STAMP_RADIUS);

        // Crosshair at the cursor
        setcolor(Color::WHITE);
        line(mx - 10, my, mx + 10, my);
        line(mx, my - 10, mx, my + 10);

        // HUD
        setcolor(Color::WHITE);
        outtextxy(10, 10, "Interactive BGI Demo (minifb)");
        outtextxy(10, 30, &format!("Mouse: ({}, {})", mx, my));
        outtextxy(
            10,
            50,
            &format!(
                "Color: {}   Shape: {}",
                colors[color_idx].0, SHAPE_NAMES[shape_idx]
            ),
        );
        outtextxy(10, 70, &format!("Stamps: {}", stamps.len()));
        outtextxy(
            10,
            getmaxy() - 40,
            "L-click: stamp  R-click/c: color  s: shape",
        );
        outtextxy(10, getmaxy() - 20, "middle/x: clear   ESC/q: quit");

        refresh();
        frame = frame.wrapping_add(1);
        delay(16); // ~60 FPS
    }

    closegraph();
    println!("Graphics closed.");
}
