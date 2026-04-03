//! Example demonstrating BGI device and mode constants
//!
//! This example shows how to use the proper BGI device and mode constants
//! for different graphics configurations.

use bgi::*;

fn main() {
    println!("BGI Device and Mode Constants Example");
    println!("====================================");

    // Example 1: VGA High Resolution (640x480, 16 colors)
    {
        let mut driver = VGA;
        let mut mode = VGAHI;

        println!("\nExample 1: VGA High Resolution");
        println!("Driver: {} (VGA)", driver);
        println!("Mode: {} (VGAHI - 640x480, 16 colors)", mode);

        initgraph(&mut driver, &mut mode, "");
        match graphresult() {
            GraphResult::Ok => {
                println!("✓ Graphics initialized successfully");
                println!("Current mode: {}", getgraphmode());
                closegraph();
            }
            error => println!("✗ Failed to initialize: {:?}", error),
        }
    }

    // Example 2: CGA Medium Resolution (320x200, 4 colors)
    {
        let mut driver = CGA;
        let mut mode = CGAMED;

        println!("\nExample 2: CGA Medium Resolution");
        println!("Driver: {} (CGA)", driver);
        println!("Mode: {} (CGAMED - 320x200, 4 colors)", mode);

        initgraph(&mut driver, &mut mode, "");
        match graphresult() {
            GraphResult::Ok => {
                println!("✓ Graphics initialized successfully");
                println!("Current mode: {}", getgraphmode());
                closegraph();
            }
            error => println!("✗ Failed to initialize: {:?}", error),
        }
    }

    // Example 3: EGA High Resolution (640x350, 16 colors)
    {
        let mut driver = EGA;
        let mut mode = EGAHI;

        println!("\nExample 3: EGA High Resolution");
        println!("Driver: {} (EGA)", driver);
        println!("Mode: {} (EGAHI - 640x350, 16 colors)", mode);

        initgraph(&mut driver, &mut mode, "");
        match graphresult() {
            GraphResult::Ok => {
                println!("✓ Graphics initialized successfully");
                println!("Current mode: {}", getgraphmode());
                closegraph();
            }
            error => println!("✗ Failed to initialize: {:?}", error),
        }
    }

    // Example 4: MCGA 256-color mode (320x200, 256 colors)
    {
        let mut driver = MCGA;
        let mut mode = MCGAHI;

        println!("\nExample 4: MCGA 256-color mode");
        println!("Driver: {} (MCGA)", driver);
        println!("Mode: {} (MCGAHI - 320x200, 256 colors)", mode);

        initgraph(&mut driver, &mut mode, "");
        match graphresult() {
            GraphResult::Ok => {
                println!("✓ Graphics initialized successfully");
                println!("Current mode: {}", getgraphmode());
                closegraph();
            }
            error => println!("✗ Failed to initialize: {:?}", error),
        }
    }

    // Example 5: Auto-detect mode
    {
        let mut driver = DETECT;
        let mut mode = 0; // Auto-detect will choose the mode

        println!("\nExample 5: Auto-detect");
        println!("Driver: {} (DETECT)", driver);
        println!("Mode: {} (auto-detect)", mode);

        initgraph(&mut driver, &mut mode, "");
        match graphresult() {
            GraphResult::Ok => {
                println!("✓ Graphics initialized successfully");
                println!("Detected driver: {}", driver);
                println!("Detected mode: {}", mode);
                println!("Current mode: {}", getgraphmode());
                closegraph();
            }
            error => println!("✗ Failed to initialize: {:?}", error),
        }
    }

    println!("\nAvailable Device Constants:");
    println!(
        "DETECT = {}, CGA = {}, MCGA = {}, EGA = {}",
        DETECT, CGA, MCGA, EGA
    );
    println!(
        "EGA64 = {}, EGAMONO = {}, IBM8514 = {}, HERCMONO = {}",
        EGA64, EGAMONO, IBM8514, HERCMONO
    );
    println!("ATT400 = {}, VGA = {}, PC3270 = {}", ATT400, VGA, PC3270);

    println!("\nAvailable Mode Constants:");
    println!(
        "CGA: CGAHI = {}, CGAMED = {}, CGALO = {}",
        CGAHI, CGAMED, CGALO
    );
    println!(
        "EGA: EGAHI = {}, EGA64LO = {}, EGAMED = {}",
        EGAHI, EGA64LO, EGAMED
    );
    println!(
        "VGA: VGALO = {}, VGAHI = {}, VGAMED = {}",
        VGALO, VGAHI, VGAMED
    );
    println!("MCGA: MCGAHI = {}", MCGAHI);

    println!("\nMode Information Query:");
    let modes = [
        (VGA, VGAHI, "VGA High"),
        (CGA, CGAMED, "CGA Medium"),
        (EGA, EGAHI, "EGA High"),
        (MCGA, MCGAHI, "MCGA High"),
    ];

    for (driver, mode, name) in modes {
        if let Some((width, height, colors)) = getmodeinfo(driver, mode) {
            println!("{}: {}x{}, {} colors", name, width, height, colors);
        }
    }
}
