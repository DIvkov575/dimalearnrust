use rstar::primitives::Rectangle;
use rstar::{RTree, RTreeObject};

// Define a struct to represent a 2D point with bounding box capabilities
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Implement RTreeObject for the Point struct
impl RTreeObject for Point {
    type Envelope = Rectangle<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        Rectangle::from_corners([self.x, self.y], [self.x, self.y])
    }
}

fn main() {
    // Create a new R-tree
    let mut tree = RTree::new();

    // Insert some points into the R-tree
    tree.insert(Point { x: 1.0, y: 2.0 });
    tree.insert(Point { x: 2.0, y: 3.0 });
    tree.insert(Point { x: 3.0, y: 4.0 });
    tree.insert(Point { x: 4.0, y: 5.0 });

    // Define a query bounding box
    let query_rect = Rectangle::from_corners([1.5, 2.5], [3.5, 4.5]);

    // Query the R-tree for points within the bounding box
    let results: Vec<&Point> = tree.locate_in_envelope_intersecting(&query_rect).collect();

    // Print the results
    println!("Points within the query rectangle:");
    for point in results {
        println!("{:?}", point);
    }
}