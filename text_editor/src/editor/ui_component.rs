pub mod command_bar;
pub mod message_bar;
pub mod status_bar;
pub mod view;

use super::terminal::TerminalSize;

pub trait UiComponent {
    /// Marks if ui component need to be redrawn
    fn mark_redraw(&mut self, needs_redraw: bool);

    /// Get status of redraw
    fn needs_redraw(&self) -> bool;

    /// Resize component
    fn resize(&mut self, new_size: TerminalSize) {
        self.set_size(new_size);
        self.mark_redraw(true);
    }

    /// Set the size of the component
    fn set_size(&mut self, new_size: TerminalSize);

    /// Draw this component if it's visible and in need of redrawing
    fn render(&mut self, origin_y: usize) {
        if self.needs_redraw() {
            match self.draw(origin_y) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }

    /// Method to actually draw the component, must be implemented by each component
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error>;
}
