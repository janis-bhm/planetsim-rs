use std::collections::BTreeMap;

use crate::body::{Body, BodyDynamics};
use crate::decimal_matrix_3d::DecimalMatrix3d;
use crate::decimal_vector_3d::DecimalVector3d;
use crate::sin_cos::PIMUL2;
use dashu_float::DBig;

#[derive(Debug, Clone)]
pub struct SimulatedBody {
    id: i32,
    body: Body,
    position: DecimalVector3d,
    velocity: DecimalVector3d,
    orientation: DecimalMatrix3d,
    parent: Option<i32>, // -1 means no
}

#[derive(Debug)]
pub struct Simulation {
    pub bodies: Vec<SimulatedBody>,
    id_counter: i32,
    bodies_tree: BTreeMap<Key, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    /// value is id of body
    StaticBody { id: i32 },
    /// value is index into bodies
    Body { id: i32 },
    /// value is id of child
    Satellite { parent: i32, child: i32 },
    /// value is id of parent
    Parent { id: i32 },
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            bodies: vec![],
            id_counter: 0,
            bodies_tree: BTreeMap::new(),
        }
    }

    fn next_id(&mut self) -> i32 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    fn add_body(&mut self, body: Body, parent: Option<i32>) -> i32 {
        let new_id = self.next_id();

        if let Some(parent) = parent {
            self.bodies_tree.insert(
                Key::Satellite {
                    parent,
                    child: new_id,
                },
                new_id as u32,
            );
            self.bodies_tree
                .insert(Key::Parent { id: new_id }, parent as u32);
        }

        if let BodyDynamics::Static(_) = &body.dynamics {
            self.bodies_tree
                .insert(Key::StaticBody { id: new_id }, new_id as u32);
        }
        for satellite in body.satellites.clone() {
            let satellite_id = self.add_body(satellite, Some(new_id));
            self.bodies_tree.insert(
                Key::Satellite {
                    parent: new_id,
                    child: satellite_id,
                },
                new_id as u32,
            );
        }

        let simulated_body = SimulatedBody {
            id: new_id,
            parent,
            body,
            position: DecimalVector3d::zero(),
            velocity: DecimalVector3d::zero(),
            orientation: DecimalMatrix3d::identity(),
        };

        self.bodies_tree
            .insert(Key::Body { id: new_id }, self.bodies.len() as u32);
        self.bodies.push(simulated_body);

        new_id
    }

    pub fn add_hierarchy(&mut self, body: Body, parent: Option<i32>) -> i32 {
        self.add_body(body, parent)
    }

    fn get_body_by_id(&self, id: i32) -> Option<&SimulatedBody> {
        self.bodies_tree
            .get(&Key::Body { id })
            .map(|&idx| &self.bodies[idx as usize])
    }

    fn get_mut_body_by_id(&mut self, id: i32) -> Option<&mut SimulatedBody> {
        self.bodies_tree
            .get(&Key::Body { id })
            .map(|&idx| &mut self.bodies[idx as usize])
    }

    fn get_all_parents_into(&self, id: i32, buf: &mut Vec<i32>) {
        if let Some(&parent) = self.bodies_tree.get(&Key::Parent { id }) {
            let parent = parent as i32;
            buf.push(parent);
            self.get_all_parents_into(parent, buf);
        }
    }
    fn get_all_children_into(&self, id: i32, buf: &mut Vec<i32>) {
        let range = self
            .bodies_tree
            .range(
                Key::Satellite {
                    parent: id,
                    child: i32::MIN,
                }..=Key::Satellite {
                    parent: id,
                    child: i32::MAX,
                },
            )
            .map(|(_, child)| *child as i32);

        for child in range {
            buf.push(child);
            self.get_all_children_into(child, buf);
        }
    }

    fn resolve_hierarchy_up(&self, body: &SimulatedBody) -> Vec<&SimulatedBody> {
        /* how this will look like for example for the moon,
         moon gets into this function, we don't want to add it
         its parent is earth, it gets found, is added to the moon-result
         then earth gets into this function,
         its parent is the sun, it gets added to the earth-result
         then sun gets into this function, but doesn't have a parent, so sun-result is []
         then sun result [] gets appended to earth result [sun] results in [sun]
         then earth result [sun] gets appended to moon result [earth] results in [earth, sun]
         so it goes upward from the body
        */

        let mut out = Vec::new();
        self.get_all_parents_into(body.id, &mut out);
        out.into_iter()
            .filter_map(|id| self.get_body_by_id(id))
            .collect()
    }

    fn resolve_hierarchy_down(&self, body: &SimulatedBody) -> Vec<&SimulatedBody> {
        /* how this will look like for example for the sun,
        sun gets into this function, its satellites are iterated, lets simplify to Venus, Earth, and Mars
        to sun result first added is [Venus]
        then venus gets into this function, but has no satellites, so nothing gets added
        then to sun result [Earth] is added
        then earth gets into this function, results in [Moon], this is appended to
        then [Mars] is added
        so in final it will look like [Venus, Earth, Moon, Mars] it's not optimal,
        but it's good for this purpose here
        */
        let mut out = Vec::new();
        self.get_all_children_into(body.id, &mut out);
        out.into_iter()
            .filter_map(|id| self.get_body_by_id(id))
            .collect()
    }

    fn get_body_position(&self, time: DBig, body_id: i32) -> DecimalVector3d {
        let body = self.get_body_by_id(body_id).unwrap();
        match &body.body.dynamics {
            BodyDynamics::Static(static_dynamics) => static_dynamics.position.clone(),
            BodyDynamics::Orbiting(orbiting) => {
                // SAFETY: we know that this body has a parent, and that the parent is in this tree.
                let parent = self.get_body_by_id(body.parent.unwrap()).unwrap();
                let parent_position = &parent.position;
                let orbit_progression = (time / &orbiting.orbit_period).fract();
                let angle = PIMUL2.clone() * orbit_progression;
                let rotation_matrix =
                    DecimalMatrix3d::axis_angle(orbiting.orbit_plane_normal.clone(), angle);
                rotation_matrix.apply(DecimalVector3d::new(
                    orbiting.orbit_radius.clone(),
                    DBig::ZERO,
                    DBig::ZERO,
                )) + parent_position
            }
        }
    }

    pub fn update(&mut self, time: DBig) {
        let mut schedule: Vec<i32> = vec![];

        for (_, &id) in self
            .bodies_tree
            .range(Key::StaticBody { id: i32::MIN }..=Key::StaticBody { id: i32::MAX })
        {
            self.get_all_children_into(id as i32, &mut schedule);
        }

        for body_id in schedule {
            let position = self.get_body_position(time.clone(), body_id);
            let pos_second_ago = self.get_body_position(time.clone() - DBig::ONE, body_id);
            let velocity = position.clone() - pos_second_ago;

            let body = self.get_mut_body_by_id(body_id).unwrap();
            body.position = position;
            body.velocity = velocity;
        }
    }
}
