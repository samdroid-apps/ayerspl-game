extern crate three;
extern crate mint;

use super::color;


const LANE_WIDTH: f32 = 0.5;
pub const ROAD_LEVEL_Z: f32 = 0.;

pub struct GroundPoint {
    x: f32,
    y: f32,
}

impl GroundPoint {
    pub fn dist_to(&self, other: &GroundPoint) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl From<[f32; 2]> for GroundPoint {
    fn from(arr: [f32; 2]) -> Self {
        GroundPoint { x: arr[0], y: arr[1] }
    }
}




pub struct Lane {
    start: GroundPoint,
    end: GroundPoint,
}

impl Lane {
    pub fn new(start: GroundPoint, end: GroundPoint) -> Lane {
        Lane {
            start,
            end,
        }
    }

    pub fn get_object(&self, factory: &mut three::Factory) -> three::Mesh {
        let dist = self.start.dist_to(&self.end);
        let dx = (self.end.x - self.start.x) / dist;
        let dy = (self.end.y - self.start.y) / dist;

        // clockwise perpendicular to the road lane line
        // make the width a little smaller so it gets a natural border
        let perpx = dy * (LANE_WIDTH * 0.9) * 0.5;
        let perpy = -dx * (LANE_WIDTH * 0.9) * 0.5;

        let vertices: Vec<mint::Point3<f32>> = vec![
            [self.start.x+perpx, self.start.y+perpy, ROAD_LEVEL_Z].into(),
            [self.start.x-perpx, self.start.y-perpy, ROAD_LEVEL_Z].into(),
            [self.end.x+perpx, self.end.y+perpy, ROAD_LEVEL_Z].into(),
            [self.end.x-perpx, self.end.y-perpy, ROAD_LEVEL_Z].into(),
        ];
        let faces = vec![
            [1, 2, 3],
            [1, 0, 2],
        ];

        let geometry = three::Geometry {
            faces,
            base: three::Shape {
                vertices,
                .. three::Shape::default()
            },
            .. three::Geometry::default()
        };
        /*
        let geometry = three::Geometry::plane(
            LANE_WIDTH,
            self.start.dist_to(&self.end));
        */
        let material = three::material::Basic {
            color: color::get_road_lane_color(),
            map: None,
        };
        let mesh = factory.mesh(geometry, material);
        mesh
    }
}
