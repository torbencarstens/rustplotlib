use format_num::NumberFormat;
use svg::Node;
use svg::node::element::{Group, Line};
use svg::node::element::Text;
use svg::node::Text as TextNode;

use crate::axis::AxisPosition;

/// A simple struct that represents an axis line.
pub struct Grid {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x_tick_spacing: f32,
    y_tick_spacing: f32,
}

impl Grid {
    /// Create a new instance of axis line.
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, x_tick_spacing: f32, y_tick_spacing: f32) -> Self {
        Self { x1, y1, x2, y2, x_tick_spacing, y_tick_spacing }
    }

    /// Render the grid to svg.
    pub fn to_svg(&self) -> Result<Vec<Group>, String> {
        let mut x = self.x1;
        let mut y = self.y1;
        let mut x_group = Group::new()
            .set("class", "x-grid-axis");
        let mut y_group = Group::new()
            .set("class", "y-grid-axis");

        while x < self.x2 {
            let line = Line::new()
                .set("x1", x)
                .set("y1", self.y1)
                .set("x2", x + 0.000001)
                .set("y2", self.y2)
                .set("shape-rendering", "crispEdges")
                .set("stroke-width", 2)
                .set("stroke", "#555555");

            x_group = x_group.add(line);
            x += self.x_tick_spacing;
        };

        while y < self.y2 {
            let line = Line::new()
                .set("x1", self.x1)
                .set("y1", y)
                .set("x2", self.x2 + 0.000001)
                .set("y2", y + 0.000001)
                .set("shape-rendering", "crispEdges")
                .set("stroke-width", 1)
                .set("stroke", "#555555");

            y_group = y_group.add(line);
            y += self.y_tick_spacing;
        };

        Ok(vec![x_group, y_group])
    }
}
