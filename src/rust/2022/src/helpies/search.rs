use glam::IVec2;
use priority_queue::priority_queue::PriorityQueue;
use std::borrow::Borrow;
use std::ops::Deref;
use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
};

pub trait Node: Sized + Hash + Eq {
    fn position(&self) -> IVec2;
    fn cost(&self) -> i32; // The cost of moving to this node
    fn f(&mut self, start_node: NodeWrapper<Self>, end_node: NodeWrapper<Self>) -> i32; // Total cost of the node
    fn g(&mut self, start_node: NodeWrapper<Self>) -> i32; // Cost of the path from the start to this node
    fn h(&mut self, end_node: NodeWrapper<Self>) -> i32; // Estimated from this node to the end
    fn visited(&self) -> bool;
    fn visited_mut(&mut self) -> &mut bool;
    fn get_parent(&self) -> &Option<NodeWrapper<Self>>;
    fn set_parent(&mut self, new_parent: Option<NodeWrapper<Self>>); /* Ur not my real dad */
    fn get_node_neighbors(&self, nodes: &Vec<Vec<NodeWrapper<Self>>>) -> Vec<NodeWrapper<Self>>;
}

struct BaseNode {
    pos_x: i32,
    pos_y: i32,
    c: i32,
    f: i32,
    g: i32,
    h: i32,
    visited: bool, // has this node been evaluated yet
    parent: Option<NodeWrapper<Self>>,
}

impl Node for BaseNode {
    fn position(&self) -> IVec2 {
        IVec2 {
            x: self.pos_x,
            y: self.pos_y,
        }
    }
    fn cost(&self) -> i32 {
        self.c
    }
    fn f(&mut self, start_node: NodeWrapper<Self>, end_node: NodeWrapper<Self>) -> i32 {
        if self.f == i32::MAX {
            self.f = self.g(start_node) + self.h(end_node);
        }
        self.f
    }
    fn g(&mut self, start_node: NodeWrapper<Self>) -> i32 {
        if self.g == i32::MAX {
            let v = start_node.0.borrow().position() - self.position();
            self.g = v.x + v.y;
        }
        self.g
    }
    fn h(&mut self, end_node: NodeWrapper<Self>) -> i32 {
        if self.h == i32::MAX {
            let v = end_node.0.borrow().position() - self.position();
            self.h = v.x + v.y;
        }
        self.h
    }
    fn visited(&self) -> bool {
        self.visited
    }
    fn visited_mut(&mut self) -> &mut bool {
        &mut self.visited
    }
    fn get_parent(&self) -> &Option<NodeWrapper<Self>> {
        &self.parent
    }
    fn set_parent(&mut self, new_parent: Option<NodeWrapper<Self>>) {
        self.parent = new_parent;
    }
    fn get_node_neighbors(&self, cells: &Vec<Vec<NodeWrapper<Self>>>) -> Vec<NodeWrapper<Self>> {
        let n: IVec2 = self.position() + IVec2 { x: 1, y: 0 };
        let e: IVec2 = self.position() + IVec2 { x: 0, y: 1 };
        let s: IVec2 = self.position() - IVec2 { x: -1, y: 0 };
        let w: IVec2 = self.position() - IVec2 { x: 0, y: -1 };
        let result: Vec<NodeWrapper<Self>> = (&[n, e, s, w])
            .iter()
            .filter(|x| {
                (0..cells.len()).contains(&(x.x as usize))
                    && (0..cells[0].len()).contains(&(x.y as usize))
            })
            .map(|v| {
                let x = v.x as usize;
                let y = v.y as usize;
                cells[x][y].clone()
            })
            .collect();
        result
    }
}

impl Default for BaseNode {
    fn default() -> Self {
        Self {
            pos_x: i32::MAX,
            pos_y: i32::MAX,
            c: i32::MAX,
            f: i32::MAX,
            g: i32::MAX,
            h: i32::MAX,
            visited: false,
            parent: None,
        }
    }
}

impl Hash for BaseNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.f.hash(state);
        self.pos_x.hash(state);
        self.pos_y.hash(state);
    }
}

impl PartialEq for BaseNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos_x == other.pos_x
            && self.pos_y == other.pos_y
            && self.c == other.c
            && self.f == other.f
            && self.g == other.g
            && self.h == other.h
            && self.visited == other.visited
    }
}

impl Eq for BaseNode {}

impl BaseNode {
    fn new(pos_x: i32, pos_y: i32) -> Self {
        Self {
            pos_x,
            pos_y,
            ..Default::default()
        }
    }
}

pub struct MountainNode {
    pos_x: i32,
    pos_y: i32,
    c: i32,
    f: i32,
    g: i32,
    h: i32,
    height: i32,
    visited: bool, // has this node been evaluated yet
    parent: Option<NodeWrapper<Self>>,
}

impl Node for MountainNode {
    fn position(&self) -> IVec2 {
        IVec2 {
            x: self.pos_x,
            y: self.pos_y,
        }
    }
    fn cost(&self) -> i32 {
        self.c
    }
    fn f(&mut self, start_node: NodeWrapper<Self>, end_node: NodeWrapper<Self>) -> i32 {
        if self.f == i32::MAX {
            self.f = self.g(start_node) + self.h(end_node);
        }
        self.f
    }
    fn g(&mut self, start_node: NodeWrapper<Self>) -> i32 {
        if self.g == i32::MAX {
            let v = start_node.0.borrow().position() - self.position();
            self.g = v.x + v.y;
        }
        self.g
    }
    fn h(&mut self, end_node: NodeWrapper<Self>) -> i32 {
        if self.h == i32::MAX {
            let v = end_node.0.borrow().position() - self.position();
            self.h = v.x + v.y;
        }
        self.h
    }
    fn visited(&self) -> bool {
        self.visited
    }
    fn visited_mut(&mut self) -> &mut bool {
        &mut self.visited
    }
    fn get_parent(&self) -> &Option<NodeWrapper<Self>> {
        &self.parent
    }
    fn set_parent(&mut self, new_parent: Option<NodeWrapper<Self>>) {
        self.parent = new_parent;
    }
    fn get_node_neighbors(&self, cells: &Vec<Vec<NodeWrapper<Self>>>) -> Vec<NodeWrapper<Self>> {
        let n: IVec2 = self.position() + IVec2 { x: 1, y: 0 };
        let e: IVec2 = self.position() + IVec2 { x: 0, y: 1 };
        let s: IVec2 = self.position() - IVec2 { x: -1, y: 0 };
        let w: IVec2 = self.position() - IVec2 { x: 0, y: -1 };
        let result: Vec<NodeWrapper<Self>> = (&[n, e, s, w])
            .iter()
            .filter(|x| {
                (0..cells.len()).contains(&(x.x as usize))
                    && (0..cells[0].len()).contains(&(x.y as usize))
            })
            .map(|v| {
                let x = v.x as usize;
                let y = v.y as usize;
                cells[x][y].clone()
            })
            .filter(|n| self.height < n.borrow().height)
            .collect();
        result
    }
}

impl Default for MountainNode {
    fn default() -> Self {
        Self {
            pos_x: i32::MAX,
            pos_y: i32::MAX,
            c: i32::MAX,
            f: i32::MAX,
            g: i32::MAX,
            h: i32::MAX,
            height: i32::MAX,
            visited: false,
            parent: None,
        }
    }
}

impl Hash for MountainNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.f.hash(state);
        self.height.hash(state);
        self.pos_x.hash(state);
        self.pos_y.hash(state);
    }
}

impl PartialEq for MountainNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos_x == other.pos_x
            && self.pos_y == other.pos_y
            && self.c == other.c
            && self.f == other.f
            && self.g == other.g
            && self.h == other.h
            && self.visited == other.visited
            && self.height == other.height
    }
}

impl Eq for MountainNode {}

impl MountainNode {
    pub fn new(pos_x: i32, pos_y: i32, height: i32) -> Self {
        Self {
            pos_x,
            pos_y,
            height,
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NodeWrapper<T: Node + Hash + PartialEq + Eq>(Rc<RefCell<T>>);

impl<T: Node + Hash + PartialEq + Eq> Deref for NodeWrapper<T> {
    type Target = Rc<RefCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Node + Hash + PartialEq + Eq> Hash for NodeWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}
impl<T: Node + Hash + PartialEq + Eq> Clone for NodeWrapper<T> {
    fn clone(&self) -> Self {
        NodeWrapper(self.0.clone())
    }
}
impl<T: Node + Hash + PartialEq + Eq> NodeWrapper<T> {
    pub fn new(inner: T) -> Self {
        NodeWrapper(Rc::new(RefCell::new(inner)))
    }
}

pub struct Grid<T>
where
    T: Node + Sized + Hash + Eq,
{
    nodes: Vec<Vec<NodeWrapper<T>>>,
    start_node: NodeWrapper<T>,
    end_node: NodeWrapper<T>,
}

impl<T> Grid<T>
where
    T: Node + Sized + Hash + Eq,
{
    pub fn new(
        nodes: Vec<Vec<NodeWrapper<T>>>,
        start_node_position: IVec2,
        end_node_position: IVec2,
    ) -> Self {
        let start_node: NodeWrapper<T> =
            nodes[start_node_position.x as usize][start_node_position.y as usize].clone();
        let end_node: NodeWrapper<T> =
            nodes[end_node_position.x as usize][end_node_position.y as usize].clone();

        Self {
            nodes,
            start_node,
            end_node,
        }
    }
}

pub struct AStarSearch<T>
where
    T: Node + Sized + Hash + Eq,
{
    queue: PriorityQueue<NodeWrapper<T>, i32>,
    grid: Grid<T>,
}

impl<T> AStarSearch<T>
where
    T: Node + Sized + Hash + Eq,
{
    pub fn new(grid: Grid<T>) -> Self {
        let mut queue: PriorityQueue<NodeWrapper<T>, i32> = PriorityQueue::new();
        queue.push(grid.start_node.clone(), 1);

        let ptr: NodeWrapper<T> = grid.start_node.clone();
        Self {
            queue,
            grid,
        }
    }

    pub fn update(&mut self) -> Option<Vec<NodeWrapper<T>>> {
        /* Still broken! */
        let node: NodeWrapper<T>;
        if let Some(handle) = self.queue.pop() {
            node = handle.0.clone();
        } else {
            panic!("heck, there's nothing left to evaluate.");
        }

        if node.borrow().visited() {
            return None;
        } else {
            *node.borrow_mut().visited_mut() = true; // wat
        }

        let neighbors: Vec<NodeWrapper<T>> = node.borrow().get_node_neighbors(&self.grid.nodes);
        for n in neighbors {
            if n.borrow().position() == self.grid.end_node.0.borrow().position() {
                /*WERE THERE BOYS*/
                let mut result = vec![self.grid.end_node.clone(), n.clone()];
                break;
            } // this is gross
            if n.borrow().visited() {
                continue; /*If this node has already been visited, it has been visited on a better path*/
            }
            n.borrow_mut().set_parent(Some(node.clone())); /* we're the first to see this node so it IS its real dad*/
            self.queue.push(
                n.clone(),
                n.borrow_mut()
                    .f(self.grid.start_node.clone(), self.grid.end_node.clone()),
            );
        }
        None
    }
}
