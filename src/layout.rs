use crate::prelude::*;

/// Defines a constraint for sizing elements within a layout.
///
/// Constraints determine how much space an element should occupy relative
/// to the available space or other elements.
#[derive(Debug, Clone)]
pub enum Constraint {
    /// Takes up a specified percentage of the total available space (0.0 to 100.0).
    /// It will shrink if necessary to fit within the available space.
    Percentage(f32),
    /// Takes up a fixed amount of space in units (e.g., characters or rows).
    /// If the available space is less than the fixed size, an error may occur.
    Fixed(u16),
    /// Takes up space within a specified minimum and maximum range.
    /// It will try to fit its content but won't go below `min` or above `max`.
    Range { min: u16, max: u16 },
    /// Takes up at least the specified minimum space, but can grow beyond it.
    Min(u16),
    /// Takes up at most the specified maximum space, but can shrink below it.
    Max(u16),
    /// Takes up all the remaining available space after other constraints have been resolved.
    /// Multiple flexible constraints will share the remaining space evenly.
    Flexible,
}

/// The possible error results that can occur during layout calculation.
#[derive(Debug, PartialEq, Eq)]
pub enum LayoutError {
    /// Indicates that at least one constraint (e.g., a `Fixed` or `Range` with too high `min`)
    /// could not fit within the allocated space.
    InsufficientSpace,

    /// Occurs when `Percentage` constraints sum up to more than 100%, or a percentage
    /// value is outside the 0.0-100.0 range.
    InvalidPercentages,

    /// Reserved for potential future conflicts where constraints are logically impossible
    /// to satisfy simultaneously (currently not explicitly triggered by `resolve_constraints`).
    ConstraintConflict,
}

/// An area that a layout element takes up.
///
/// Represents a rectangular region on the screen, defined by its top-left
/// corner (x, y) and its dimensions (width, height).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    /// The X-coordinate of the top-left corner.
    pub x: u16,
    /// The Y-coordinate of the top-left corner.
    pub y: u16,
    /// The width of the rectangle.
    pub width: u16,
    /// The height of the rectangle.
    pub height: u16,
}

impl Rect {
    /// Creates a new `Rect` with the specified position and dimensions.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns the top-left position as a Vec2.
    pub fn position(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    /// Returns the size as a Vec2.
    pub fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
    }

    /// Returns the bottom-right corner as a Vec2.
    pub fn bottom_right(&self) -> Vec2 {
        vec2(self.x + self.width, self.y + self.height)
    }

    /// Returns the center point as a Vec2.
    pub fn center(&self) -> Vec2 {
        vec2(self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Creates a Rect from two Vec2 points.
    pub fn from_corners(top_left: Vec2, bottom_right: Vec2) -> Self {
        Self {
            x: top_left.x,
            y: top_left.y,
            width: bottom_right.x.saturating_sub(top_left.x),
            height: bottom_right.y.saturating_sub(top_left.y),
        }
    }

    /// Creates a Rect from a position and size.
    pub fn from_pos_size(pos: Vec2, size: Vec2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    /// Creates a new Rect with padding applied inward.
    pub fn with_padding(&self, padding: u16) -> Self {
        Self {
            x: self.x + padding,
            y: self.y + padding,
            width: self.width.saturating_sub(padding * 2),
            height: self.height.saturating_sub(padding * 2),
        }
    }

    /// Creates a new Rect with specific padding on each side.
    pub fn with_padding_sides(&self, top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            x: self.x + left,
            y: self.y + top,
            width: self.width.saturating_sub(left + right),
            height: self.height.saturating_sub(top + bottom),
        }
    }
}

impl From<Rect> for Vec2 {
    fn from(rect: Rect) -> Self {
        vec2(rect.x, rect.y)
    }
}

/// Creates a `Constraint::Percentage` variant.
pub fn percent(value: f32) -> Constraint {
    Constraint::Percentage(value)
}

/// Creates a `Constraint::Fixed` variant.
pub fn fixed(value: u16) -> Constraint {
    Constraint::Fixed(value)
}

/// Creates a `Constraint::Range` variant.
pub fn range(min_val: u16, max_val: u16) -> Constraint {
    Constraint::Range {
        min: min_val,
        max: max_val,
    }
}

/// Creates a `Constraint::Min` variant.
pub fn min(value: u16) -> Constraint {
    Constraint::Min(value)
}

/// Creates a `Constraint::Max` variant.
pub fn max(value: u16) -> Constraint {
    Constraint::Max(value)
}

/// Creates a `Constraint::Flexible` variant.
pub fn flexible() -> Constraint {
    Constraint::Flexible
}

/// Defines a horizontal and vertical grid layout setup.
///
/// `Layout` is used for separating a given total space (e.g., the window size)
/// into easy-to-manage rectangular chunks for rendering UI elements.
#[derive(Default, Debug, Clone)]
pub struct Layout {
    /// A vector where each tuple represents a row: `(height_constraint, width_constraints_for_columns)`.
    rows: Vec<(Constraint, Vec<Constraint>)>,
}

impl Layout {
    /// Starts a new `Layout` definition.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new row to the layout with specified height and column width constraints.
    pub fn row(
        mut self,
        height_constraint: Constraint,
        width_constraints: Vec<Constraint>,
    ) -> Self {
        self.rows.push((height_constraint, width_constraints));
        self
    }

    /// Creates a row that takes up the full width of the available space with a single height constraint.
    pub fn empty_row(self, constraint: Constraint) -> Self {
        self.row(constraint, vec![flexible()])
    }

    /// Calculates the `Rect`s for all elements in the layout based on the total available space.
    pub fn calculate(self, space: impl Into<Vec2>) -> Result<Vec<Vec<Rect>>, LayoutError> {
        calculate_layout(space, self.rows)
    }

    /// Calculates the layout and renders elements to each rect area.
    pub fn render<R: Render>(
        self,
        space: impl Into<Vec2>,
        buffer: &mut Buffer,
        elements: Vec<Vec<R>>,
    ) -> Result<Vec<Vec<Rect>>, LayoutError> {
        let rects = self.calculate(space)?;

        for (row_idx, row_rects) in rects.iter().enumerate() {
            if let Some(row_elements) = elements.get(row_idx) {
                for (col_idx, rect) in row_rects.iter().enumerate() {
                    if let Some(element) = row_elements.get(col_idx) {
                        element.render(rect.position(), buffer);
                    }
                }
            }
        }

        Ok(rects)
    }

    /// Calculates the layout and renders elements with clipping to fit within each rect.
    pub fn render_clipped<R: Render>(
        self,
        space: impl Into<Vec2>,
        buffer: &mut Buffer,
        elements: Vec<Vec<R>>,
    ) -> Result<Vec<Vec<Rect>>, LayoutError> {
        let rects = self.calculate(space)?;

        for (row_idx, row_rects) in rects.iter().enumerate() {
            if let Some(row_elements) = elements.get(row_idx) {
                for (col_idx, rect) in row_rects.iter().enumerate() {
                    if let Some(element) = row_elements.get(col_idx) {
                        element.render_clipped(rect.position(), rect.size(), buffer);
                    }
                }
            }
        }

        Ok(rects)
    }
}

/// A helper for working with a single calculated layout.
///
/// This provides convenient methods for accessing and working with layout rects.
pub struct CalculatedLayout {
    rects: Vec<Vec<Rect>>,
}

impl CalculatedLayout {
    /// Creates a new CalculatedLayout from calculated rects.
    pub fn new(rects: Vec<Vec<Rect>>) -> Self {
        Self { rects }
    }

    /// Gets a rect at the specified row and column.
    pub fn get(&self, row: usize, col: usize) -> Option<&Rect> {
        self.rects.get(row)?.get(col)
    }

    /// Gets all rects in a row.
    pub fn row(&self, row: usize) -> Option<&[Rect]> {
        self.rects.get(row).map(|r| r.as_slice())
    }

    /// Returns the total number of rows.
    pub fn row_count(&self) -> usize {
        self.rects.len()
    }

    /// Returns the number of columns in a specific row.
    pub fn col_count(&self, row: usize) -> usize {
        self.rects.get(row).map(|r| r.len()).unwrap_or(0)
    }

    /// Iterates over all rects with their row and column indices.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &Rect)> {
        self.rects.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_idx, rect)| (row_idx, col_idx, rect))
        })
    }

    /// Renders an element at a specific layout position.
    pub fn render_at<R: Render>(
        &self,
        row: usize,
        col: usize,
        element: R,
        buffer: &mut Buffer,
    ) -> Option<Vec2> {
        let rect = self.get(row, col)?;
        Some(element.render(rect.position(), buffer))
    }

    /// Renders an element clipped to a specific layout position.
    pub fn render_clipped_at<R: Render>(
        &self,
        row: usize,
        col: usize,
        element: R,
        buffer: &mut Buffer,
    ) -> Option<Vec2> {
        let rect = self.get(row, col)?;
        Some(element.render_clipped(rect.position(), rect.size(), buffer))
    }
}

/// Calculates the layout of a grid, resolving constraints for rows and columns.
pub fn calculate_layout(
    total_space: impl Into<Vec2>,
    rows: Vec<(Constraint, Vec<Constraint>)>,
) -> Result<Vec<Vec<Rect>>, LayoutError> {
    let total_space = total_space.into();
    let height_constraints: Vec<Constraint> = rows.iter().map(|(h, _)| h.clone()).collect();

    // Resolve heights for all rows
    let row_heights = resolve_constraints(&height_constraints, total_space.y)?;
    let mut result = Vec::new();
    let mut current_y = 0u16;

    // Iterate through rows to resolve column widths and create Rects
    for (row_idx, (_, width_constraints)) in rows.iter().enumerate() {
        let row_height = row_heights[row_idx];
        let widths = resolve_constraints(width_constraints, total_space.x)?;

        let mut row_elements = Vec::new();
        let mut current_x = 0u16;

        for width in widths {
            row_elements.push(Rect::new(current_x, current_y, width, row_height));
            current_x += width;
        }

        result.push(row_elements);
        current_y += row_height;
    }

    Ok(result)
}

/// Resolves a list of `Constraint`s for a single dimension (either width or height).
pub fn resolve_constraints(
    constraints: &[Constraint],
    available: u16,
) -> Result<Vec<u16>, LayoutError> {
    if constraints.is_empty() {
        return Ok(vec![]);
    }

    let mut total_percentage = 0.0f32;
    for constraint in constraints {
        if let Constraint::Percentage(pct) = constraint {
            if *pct < 0.0 || *pct > 100.0 {
                return Err(LayoutError::InvalidPercentages);
            }
            total_percentage += pct;
        }
    }

    if total_percentage > 100.0 {
        return Err(LayoutError::InvalidPercentages);
    }

    let mut allocated_sizes = vec![0u16; constraints.len()];

    // Allocate fixed sizes first
    let mut fixed_total = 0u32;
    for (i, constraint) in constraints.iter().enumerate() {
        if let Constraint::Fixed(size) = constraint {
            allocated_sizes[i] = *size;
            fixed_total += *size as u32;
        }
    }

    if fixed_total > available as u32 {
        return Err(LayoutError::InsufficientSpace);
    }

    // Allocate percentage sizes
    let mut percentage_total = 0u32;
    for (i, constraint) in constraints.iter().enumerate() {
        if let Constraint::Percentage(pct) = constraint {
            let ideal_size = ((available as f32 * pct) / 100.0).round() as u32;
            allocated_sizes[i] = ideal_size as u16;
            percentage_total += ideal_size;
        }
    }

    // If combined fixed and percentage exceeds available, shrink percentages proportionally
    if fixed_total + percentage_total > available as u32 {
        let shrink_factor = (available as u32 - fixed_total) as f32 / percentage_total as f32;
        for (i, constraint) in constraints.iter().enumerate() {
            if let Constraint::Percentage(_) = constraint {
                allocated_sizes[i] = (allocated_sizes[i] as f32 * shrink_factor).round() as u16;
            }
        }
    }

    // Ensure minimums are met for Range and Min constraints
    for (i, constraint) in constraints.iter().enumerate() {
        match constraint {
            Constraint::Range { min: min_val, .. } | Constraint::Min(min_val) => {
                allocated_sizes[i] = allocated_sizes[i].max(*min_val);
            }
            _ => {}
        }
    }

    let used_space: u32 = allocated_sizes.iter().map(|&x| x as u32).sum();

    if used_space > available as u32 {
        return Err(LayoutError::InsufficientSpace);
    }

    let mut remaining_space = (available as u32) - used_space;

    // Identify indices of flexible, min, max, and range constraints for expansion
    let mut expandable_indices: Vec<(usize, u16)> = Vec::new();

    for (i, constraint) in constraints.iter().enumerate() {
        let max_val = match constraint {
            Constraint::Range { max: m, .. } => Some(*m),
            Constraint::Max(m) => Some(*m),
            Constraint::Min(_) => Some(u16::MAX),
            Constraint::Flexible => Some(u16::MAX),
            _ => None,
        };

        if let Some(max) = max_val {
            expandable_indices.push((i, max));
        }
    }

    // Distribute remaining space to expandable constraints
    if !expandable_indices.is_empty() && remaining_space > 0 {
        while remaining_space > 0 {
            let mut distributed = 0u32;
            let eligible: Vec<_> = expandable_indices
                .iter()
                .filter(|(idx, max_val)| allocated_sizes[*idx] < *max_val)
                .collect();

            if eligible.is_empty() {
                break;
            }

            let space_per_item = std::cmp::max(1, remaining_space / eligible.len() as u32);

            for &&(idx, max_val) in &eligible {
                if remaining_space == 0 {
                    break;
                }

                let can_add = std::cmp::min(
                    max_val.saturating_sub(allocated_sizes[idx]) as u32,
                    std::cmp::min(space_per_item, remaining_space),
                );

                allocated_sizes[idx] += can_add as u16;
                distributed += can_add;
                remaining_space -= can_add;
            }

            if distributed == 0 {
                break;
            }
        }
    }

    Ok(allocated_sizes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_plus_fixed_heights() {
        let layout_result = Layout::new()
            .row(percent(100.0), vec![percent(100.0)])
            .row(fixed(5), vec![percent(100.0)])
            .calculate((100, 100))
            .unwrap();
        assert_eq!(
            layout_result,
            vec![
                vec![Rect::new(0, 0, 100, 95)],
                vec![Rect::new(0, 95, 100, 5)]
            ]
        );
    }

    #[test]
    fn test_even_flexible_split() {
        let layout_result = Layout::new()
            .row(flexible(), vec![flexible(), flexible()])
            .row(flexible(), vec![flexible(), flexible()])
            .calculate((100, 100))
            .unwrap();
        assert_eq!(
            layout_result,
            vec![
                vec![Rect::new(0, 0, 50, 50), Rect::new(50, 0, 50, 50)],
                vec![Rect::new(0, 50, 50, 50), Rect::new(50, 50, 50, 50)]
            ]
        );
    }

    #[test]
    fn test_rect_helpers() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.position(), vec2(10, 20));
        assert_eq!(rect.size(), vec2(30, 40));
        assert_eq!(rect.bottom_right(), vec2(40, 60));
        assert_eq!(rect.center(), vec2(25, 40));
    }

    #[test]
    fn test_rect_padding() {
        let rect = Rect::new(10, 10, 30, 30);
        let padded = rect.with_padding(5);
        assert_eq!(padded, Rect::new(15, 15, 20, 20));
    }

    #[test]
    fn test_rect_from_corners() {
        let rect = Rect::from_corners(vec2(10, 20), vec2(40, 60));
        assert_eq!(rect, Rect::new(10, 20, 30, 40));
    }

    #[test]
    fn test_min_constraint() {
        let sizes = resolve_constraints(&[min(30), min(20)], 100).unwrap();
        assert_eq!(sizes, vec![55, 45]); // Remaining space distributed
    }

    #[test]
    fn test_max_constraint() {
        let sizes = resolve_constraints(&[max(30), flexible()], 100).unwrap();
        assert_eq!(sizes, vec![30, 70]);
    }

    #[test]
    fn test_min_insufficient() {
        let result = resolve_constraints(&[min(60), min(60)], 100);
        assert_eq!(result, Err(LayoutError::InsufficientSpace));
    }
}
