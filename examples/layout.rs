use ascii_forge::{prelude::*, widgets::Border};
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    // Will init the window for you, handling all required procedures.
    let mut window = Window::init()?;
    // Ask the system to handle panics for us.
    handle_panics();

    loop {
        window.update(Duration::from_millis(200))?;
        let window_size = window.size();

        // 1. Define the Layout structure, using Layout::row for both vertical and horizontal splits.
        let layout = Layout::new()
            // Row 1: Header (fixed height, single column)
            // Height: fixed(3) | Columns: [flexible() (100% width)]
            .row(fixed(3), vec![flexible()])
            // Row 2: Main Content (flexible height, horizontally split into two columns)
            // Height: flexible() | Columns: [fixed(20) (Sidebar width), flexible() (Main Panel width)]
            .row(flexible(), vec![fixed(20), flexible()])
            // Row 3: Footer (fixed height, single column)
            // Height: fixed(2) | Columns: [flexible() (100% width)]
            .row(fixed(2), vec![flexible()]);

        // 2. Calculate the Rectangles (Rects) for the entire grid layout
        let rects = match layout.calculate(window_size) {
            Ok(r) => r,
            Err(e) => {
                // Handle terminal size errors, e.g., if a fixed size is too big
                render!(window, vec2(0, 0) => [ format!("Layout Error: {:?}", e).red() ]);
                continue;
            }
        };

        // --- Layout Breakdown ---
        // Row 0, Column 0: Header Area
        let header_rect = rects[0][0];
        // Row 1, Column 0: Sidebar Area
        let sidebar_rect = rects[1][0];
        // Row 1, Column 1: Main Panel Area
        let main_panel_rect = rects[1][1];
        // Row 2, Column 0: Footer Area
        let footer_rect = rects[2][0];

        // 3. Render all Borders and Content

        // --- Header (Row 0) ---
        let header_border = Border::double(header_rect.width, header_rect.height)
            .with_title(" Complex Application Header ".yellow().on_blue());
        let header_inner = render!(window, header_rect.position() => [ header_border ]);
        render!(window, header_inner => [ "A Multi-Column Layout Example (Not Interactive)".white().bold() ]);

        // --- Sidebar (Row 1, Col 0) ---
        let sidebar_border =
            Border::rounded(sidebar_rect.width, sidebar_rect.height).with_title(" Nav ".magenta());
        let sidebar_inner = render!(window, sidebar_rect.position() => [ sidebar_border ]);
        render!(
            window,
            sidebar_inner => [ "Home".bold() ],
            sidebar_inner + vec2(0, 1) => [ "Settings" ],
            sidebar_inner + vec2(0, 2) => [ "Help" ],
        );

        // --- Main Panel (Row 1, Col 1) ---
        let main_panel_border = Border::square(main_panel_rect.width, main_panel_rect.height)
            .with_title(" Main Content Panel ".green());
        let main_panel_inner = render!(window, main_panel_rect.position() => [ main_panel_border ]);
        render!(
            window,
            main_panel_inner => [ "This area is the main view." ],
            main_panel_inner + vec2(0, 1) => [ "It takes up all flexible space." ],
        );

        // --- Footer (Row 2) ---
        let footer_border =
            Border::thick(footer_rect.width, footer_rect.height).with_title(" Status ".cyan());
        let footer_inner_pos = render!(window, footer_rect.position() => [ footer_border ]);
        render!(window, footer_inner_pos + vec2(1, 0) => [ "Press 'Enter' to exit.".red() ]);

        // Check if the Enter Key was pressed, and exit the app if it was.
        if event!(window, Event::Key(e) => e.code == KeyCode::Enter) {
            break;
        }
    }

    // Restore the window
    window.restore()
}
