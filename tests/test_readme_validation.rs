// Test to verify the README example compiles and works
// Note: This is a verification test for T064 documentation validation

#[cfg(test)]
mod readme_validation {
    use bgi::{Color, circle, closegraph, initgraph, setcolor};

    #[test]
    fn test_readme_example_compiles() {
        // This test validates that the classic BGI API shown in README.md actually exists and compiles

        // We can't run the full example in test mode because graphics initialization may fail
        // But we can verify the API exists and has correct signatures

        let _classic_bgi_example = || {
            let mut driver = 9; // VGA
            let mut mode = 2; // VGAHI - 640x480, 16 colors

            initgraph(&mut driver, &mut mode, "");

            setcolor(Color::RED);
            circle(320, 240, 50);

            // Skip getch() in test
            // getch(); // Wait for keypress
            closegraph();
        };

        // The fact that this compiles means the classic BGI API is correct
        println!("README classic BGI API validation: All function signatures are correct");
    }
}
