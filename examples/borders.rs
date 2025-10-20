use ascii_forge::{prelude::*, widgets::Border};
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    // Will init the window for you, handling all required procedures.
    let mut window = Window::init()?;
    // Ask the system to handle panics for us.
    handle_panics();

    loop {
        // Ask the window to draw, handle events, and fix sizing issues.
        // Duration is the time for which to poll events before re-rendering.
        window.update(Duration::from_millis(200))?;

        // Create borders with different styles
        let square_border = Border::square(30, 5)
            .with_title("Square Border")
            .with_style(ContentStyle::new().cyan());

        let rounded_border = Border::rounded(30, 5)
            .with_title("Rounded Border".red())
            .with_style(ContentStyle::new().green());

        let thick_border = Border::thick(30, 5)
            .with_title("Thick Border".blue())
            .with_style(ContentStyle::new().yellow());

        let double_border = Border::double(30, 5)
            .with_title("Double Border".on_green().black())
            .with_style(ContentStyle::new().magenta());

        // A border with a very long title to demonstrate clipping
        let clipped_border = Border::rounded(25, 4)
            .with_title("This is a very long title that will be clipped!")
            .with_style(ContentStyle::new().red());

        // Render all the borders
        render!(
            window,
            vec2(2, 1) => [ square_border ],
            vec2(2, 7) => [ rounded_border ],
            vec2(2, 13) => [ thick_border ],
            vec2(2, 19) => [ double_border ],
            vec2(35, 1) => [ clipped_border ],
            vec2(35, 7) => [ "Borders can have:".white() ],
            vec2(35, 8) => [ "• Custom styles".dark_grey() ],
            vec2(35, 9) => [ "• Titles".dark_grey() ],
            vec2(35, 10) => [ "• Different border types".dark_grey() ],
            vec2(2, 25) => [ "Press `Enter` to exit!".red() ],
        );

        // You can also render content inside borders
        let content_border = Border::square(30, 8)
            .with_title("Content Inside")
            .with_style(ContentStyle::new().blue());

        let border_loc = render!(window, vec2(35, 12) => [ content_border ]);

        render!(
            window,
            border_loc => [ "Hello from inside!".white() ],
            border_loc + vec2(0, 1) => [ "Content can be".dark_grey() ],
            border_loc + vec2(0, 2) => [ "rendered inside".dark_grey() ],
            border_loc + vec2(0, 3) => [ "the border area.".dark_grey() ],
        );

        // Check if the Enter Key was pressed, and exit the app if it was.
        if event!(window, Event::Key(e) => e.code == KeyCode::Enter) {
            break;
        }
    }

    // Restore the window, enabling the window to function normally again
    // If nothing will be run after this, once the window is dropped, this will be run implicitly.
    window.restore()
}
