/// Example demonstrating high-DPI support with scale factor
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use soft_ratatui::SoftBackend;

fn main() {
    // Create backend with 2.0 scale factor for high-DPI displays (e.g., Retina displays)
    let backend = SoftBackend::new_with_system_fonts_and_scale(100, 50, 16, 2.0);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    terminal.draw(|frame| {
        let area = frame.area();
        let text = format!(
            "High-DPI Example\n\
            Window area: {}\n\
            Scale factor: 2.0\n\n\
            This text should appear sharp on high-DPI displays.\n\
            The backend renders at 2x resolution internally.",
            area
        );
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::new().title("High-DPI Rendering").borders(Borders::ALL))
                .wrap(Wrap { trim: false }),
            area,
        );
    }).unwrap();

    // The pixmap data is now at 2x resolution
    let backend = terminal.backend();
    let width = backend.get_pixmap_width();
    let height = backend.get_pixmap_height();
    println!("Logical size: 100x50 cells");
    println!("Physical pixmap size: {}x{} pixels", width, height);
}