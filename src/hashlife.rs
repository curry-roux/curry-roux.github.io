use std::{
    hash::{Hash, Hasher}, 
    ops::{Add, AddAssign, Sub, SubAssign},
    cmp::{Ordering},
};
use bimap::BiMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(u8);

impl Level {
    pub const MAX_LEVEL: Self = Self(63);
    pub const LEAF_LEVEL: Self = Self(0);

    pub fn new(n: u8) -> Self {
        Self(n)
    }

    pub const fn side_len(self) -> u64 {
        1 << self.0 
    }

    pub fn quadrant_center(self, quadrant: Quadrant) -> Position {
        let delta = i64::try_from(self.side_len() / 4).unwrap();
        match quadrant {
            Quadrant::NorthWest => (-delta, -delta).into(),
            Quadrant::NorthEast => (delta, -delta).into(),
            Quadrant::SouthWest => (-delta, delta).into(),
            Quadrant::SouthEast => (delta, delta).into(),
        }
    }

    pub const fn min_coord(self) -> i64 {
        -(1 << (self.0 - 1)) - 1
    }

    pub const fn max_coord(self) -> i64 {
        (1 << (self.0 - 1)) - 1
    }

    pub const fn coord_range(self) -> std::ops::Range<i64> {
        self.min_coord()..self.max_coord()
    }

    pub fn min_pos(self) -> Position {
        let min = Self::min_coord(self);
        (min, min).into()
    }

    pub fn max_pos(self) -> Position {
        let max = Self::max_coord(self);
        (max, max).into()
    }

    pub fn max_steps(self) -> u64 {
        debug_assert!(self.0 >= 2, "inode evolution is level 2 or higher");
        1u64 << (self.0 - 2)
    }

    fn check_validity(self) {
        if self > Self::MAX_LEVEL {
            panic!("the maximal level ({}) was exceeded", Self::MAX_LEVEL.0);
        }
    }
}

impl PartialEq<u8> for Level {
    fn eq(&self, n: &u8) -> bool {
        self.0 == *n
    }
}

impl PartialOrd<u8> for Level {
    fn partial_cmp(&self, n: &u8) -> Option<Ordering> {
        Some(self.0.cmp(n))
    }
}

impl Add for Level {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let l = Level(self.0 + other.0);
        l.check_validity();
        l
    }
}

impl Add<u8> for Level {
    type Output = Self;

    fn add(self, n: u8) -> Self {
        let l = Level(self.0 + n);
        l.check_validity();
        l
    }
}

impl Sub for Level {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Level(self.0 - other.0)
    }
}

impl Sub<u8> for Level {
    type Output = Self;

    fn sub(self, n: u8) -> Self {
        Level(self.0 - n)
    }
}

impl SubAssign for Level {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl SubAssign<u8> for Level {
    fn sub_assign(&mut self, n: u8) {
        self.0 -= n;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub const ORIGIN: Self = Self::new(0,0);

    pub const fn new(x: i64, y:i64) -> Self {
        Self {x,y}
    }

    pub fn quadrant(self) -> Quadrant {
        match(self.x < 0, self.y < 0) {
            (true , true) => Quadrant::NorthWest,
            (false, true) => Quadrant::NorthEast,
            (true, false) => Quadrant::SouthWest,
            (false, false)=> Quadrant::SouthEast,
        }
    }

    pub fn relative_to(self, other: Self) -> Self {
        self + Offset::new(-other.x, -other.y)
    }

    pub fn in_bounds(self, level: Level) -> bool {
        let bounds = level.coord_range();
        bounds.contains(&self.x) && bounds.contains(&self.y)
    }
}

impl From<(i64, i64)> for Position {
    fn from(t: (i64, i64)) -> Position {
        Self::new(t.0, t.1)
    }
}
impl From<(i64, i64)> for Offset {
    fn from(t: (i64, i64)) -> Self {
        Self::new(t.0, t.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset {
    pub dx: i64,
    pub dy: i64,
}

impl Add for Offset {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Offset::new(self.dx + other.dy, self.dy + other.dy)
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, other: Self) {
        self.dx += other.dx;
        self.dy += other.dy;
    }
}

impl SubAssign for Offset {
    fn sub_assign(&mut self, other: Self) {
        self.dx -= other.dx;
        self.dy -= other.dy;
    }
}

impl Add<Offset> for Position {
    type Output = Self;
    fn add(self, other: Offset) -> Self::Output {
        Position::new(self.x + other.dx, self.y + other.dy)
    }
}

impl AddAssign<Offset> for Position {
    fn add_assign(&mut self, other: Offset) {
        self.x += other.dx;
        self.y += other.dy;
    }
}

impl Sub<Offset> for Position {
    type Output = Self;
    fn sub(self, other: Offset) -> Self::Output {
        Position::new(self.x - other.dx, self.y - other.dy)
    }
}

impl SubAssign<Offset> for Position {
    fn sub_assign(&mut self, other: Offset) {
        self.x -= other.dx;
        self.y -= other.dy;
    }
}

impl Offset {
    pub const fn new(dx: i64, dy: i64) -> Self {
        Self {dx, dy}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quadrant {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(usize);

impl Id {
    fn node(self, univ: &Universe) -> &Node {
        univ.table.get_by_left(&self).unwrap()
    }

    fn leaf(self, univ: &Universe) -> &Leaf {
        if let Node::Leaf(leaf) = self.node(univ) {
            leaf
        } else {
            panic!("not a leaf")
        }
    }

    fn inode(self, univ: &Universe) -> &Inode {
        if let Node::Inode(inode) = self.node(univ) {
            inode
        } else {
            panic!("not an inode")
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    Dead = 0u8,
    Alive = 1u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Leaf (pub Cell);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Leaf(Leaf),
    Inode(Inode),
}

impl From<Leaf> for Node {
    fn from(cell: Leaf) -> Self {
        Node::Leaf(cell)
    }
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub level: Level,
    pub population: u32,
    pub result: Option<Id>,
    pub nw: Id,
    pub ne: Id,
    pub sw: Id,
    pub se: Id,
}

impl PartialEq for Inode {
    fn eq(&self, other: &Self) -> bool{
        self.nw == other.nw && self.ne == other.ne && self.sw == other.sw && self.se == other.se
    }
}
impl Eq for Inode {}

impl Hash for Inode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nw.hash(state);
        self.ne.hash(state);
        self.sw.hash(state);
        self.se.hash(state);
    }
}

impl From<Inode> for Node {
    fn from(inode: Inode) -> Self {
        Node::Inode(inode)
    }
}

impl Leaf {
    pub fn new(cell: Cell) -> Self {
        Self(cell)
    }

    fn alive(self) -> bool {
        match self.0 { // ここself.0なんだ
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

impl Node {
    pub fn population(&self) -> u32 {
        match self {
            Node::Inode(ref i) => i.population,
            Node::Leaf(c) => c.0 as u32,
        }
    }

    pub fn level(&self) -> Level {
        match *self {
            Node::Inode(ref i) => i.level,
            Node::Leaf(_) => Level::LEAF_LEVEL,
        }
    }
}
#[derive(Default)]
pub struct Universe {
    table: BiMap<Id, Node>,
    root: Option<Id>,
    generation: usize,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            table: BiMap::new(),
            root: None, 
            generation: 0,
        }
    }

    pub fn initilaize(&mut self) {
        self.root = Some(self.new_empty_tree(Level::new(3)))
    }

    fn get_id(&mut self, node: Node) -> Id {
        if let Some(id) = self.table.get_by_right(&node)
        {
            *id 
        } else {
            let id = Id(self.table.len());
            self.table.insert(id, node);
            id
        }
    }

    fn new_leaf(&mut self, cell: Cell) -> Id {
        let node = Node::Leaf(Leaf::new(cell));
        self.get_id(node)
    }

    fn new_inode(&mut self, nwx: Id, nex: Id, swx: Id, sex: Id) -> Id {
        let childs = (
            nwx.node(self),
            nex.node(self),
            swx.node(self),
            sex.node(self),
        );

        let inode = match childs {
            (Node::Inode(nw), Node::Inode(ne), Node::Inode(sw), Node::Inode(se)) => {
                debug_assert!(nw.level == ne.level && ne.level == sw.level && sw.level == se.level);
                Inode {
                    level: nw.level + 1,
                    population: nw.population + ne.population + sw.population + se.population,
                    result: None,
                    nw: nwx,
                    ne: nex,
                    sw: swx,
                    se: sex,
                }
            }
            (Node::Leaf(nw), Node::Leaf(ne), Node::Leaf(sw), Node::Leaf(se)) => Inode {
                level: Level::new(1),
                population: [nw, ne, sw, se]
                    .iter()
                    .filter(|c| matches!(c.0, Cell::Alive))
                    .count() as u32,
                result: None,
                nw: nwx,
                ne: nex,
                sw: swx,
                se: sex,
            },
            _ => unreachable!(),
        };

        self.get_id(Node::Inode(inode))
    }

    fn new_empty_tree(&mut self, level: Level) -> Id {
        if level == Level::LEAF_LEVEL {
            self.new_leaf(Cell::Dead)
        } else {
            let child = Self::new_empty_tree(self, level - 1);
            self.new_inode(child, child, child, child)
        }
    }

    fn get_tree_cell(&self, tree: Id, pos: impl Into<Position>) -> Cell {
        let pos = pos.into();
        match tree.node(self) {
            Node::Leaf(c) => c.0,
            Node::Inode(Inode {
                level,
                population: _,
                result: _,
                nw,
                ne,
                sw,
                se,
            }) => match pos.quadrant() {
                Quadrant::NorthWest => {
                    self.get_tree_cell(*nw, pos.relative_to(level.quadrant_center(Quadrant::NorthWest)))
                }
                Quadrant::NorthEast => {
                    self.get_tree_cell(*ne, pos.relative_to(level.quadrant_center(Quadrant::NorthEast)))
                }
                Quadrant::SouthWest => {
                    self.get_tree_cell(*sw, pos.relative_to(level.quadrant_center(Quadrant::SouthWest)))
                }
                Quadrant::SouthEast => {
                    self.get_tree_cell(*se, pos.relative_to(level.quadrant_center(Quadrant::SouthEast)))
                }
            },
        }
    }

fn set_tree_cell(&mut self, tree: Id, pos: impl Into<Position>, state: Cell) -> Id {
        let pos = pos.into();

        match *tree.node(self) {
            Node::Leaf(_) => self.new_leaf(state),
            Node::Inode(Inode {
                level,
                population: _,
                result: _,
                nw,
                ne,
                sw,
                se,
            }) => match pos.quadrant() {
                Quadrant::NorthWest => {
                    let nw = self.set_tree_cell(
                        nw,
                        pos.relative_to(level.quadrant_center(Quadrant::NorthWest)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
                Quadrant::NorthEast => {
                    let ne = self.set_tree_cell(
                        ne,
                        pos.relative_to(level.quadrant_center(Quadrant::NorthEast)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
                Quadrant::SouthWest => {
                    let sw = self.set_tree_cell(
                        sw,
                        pos.relative_to(level.quadrant_center(Quadrant::SouthWest)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
                Quadrant::SouthEast => {
                    let se = self.set_tree_cell(
                        se,
                        pos.relative_to(level.quadrant_center(Quadrant::SouthEast)),
                        state,
                    );
                    self.new_inode(nw, ne, sw, se)
                }
            },
        }
    }

    fn expand(&mut self) {
        let level = self.root.unwrap().inode(self).level;
        let border = self.new_empty_tree(level - 1);
        let (root_nw, root_ne, root_sw, root_se) = {
            let root = self.root.unwrap().inode(self);
            (root.nw, root.ne, root.sw, root.se)
        };
        let (nw, ne, sw, se) = (
            self.new_inode(border, border, border, root_nw),
            self.new_inode(border, border, root_ne, border),
            self.new_inode(border, root_sw, border, border),
            self.new_inode(root_se, border, border, border),
        );
        self.root = Some(self.new_inode(nw, ne, sw, se));
    }

    fn evolve_tree(&mut self, tree: Id) -> Id {
        {
            let inode = tree.inode(self);
            debug_assert!(inode.level >= Level::new(2), "must be level 2 or higher");
        }

        if let Some(result) = tree.inode(self).result {
            return result;
        }

        if tree.inode(self).level == 2 {
            self.manual_evolve(tree)
        } else {
            let (tree_nw, tree_ne, tree_sw, tree_se) = {
                let inode = tree.inode(self);
                (inode.nw, inode.ne, inode.sw, inode.se)
            };
            let n00 = self.centered_sub(tree_nw);
            let n01 = self.centered_horizontal(tree_nw, tree_ne);
            let n02 = self.centered_sub(tree_ne);
            let n10 = self.centered_vertical(tree_nw, tree_sw);
            let n11 = self.centered_subsub(tree);
            let n12 = self.centered_vertical(tree_ne, tree_se);
            let n20 = self.centered_sub(tree_sw);
            let n21 = self.centered_horizontal(tree_sw, tree_se);
            let n22 = self.centered_sub(tree_se);

            let (nw, ne, sw, se) = {
                let nw = self.new_inode(n00, n01, n10, n11);
                let ne = self.new_inode(n01, n02, n11, n12);
                let sw = self.new_inode(n10, n11, n20, n21);
                let se = self.new_inode(n11, n12, n21, n22);
                (
                    self.evolve_tree(nw),
                    self.evolve_tree(ne),
                    self.evolve_tree(sw),
                    self.evolve_tree(se),
                )
            };
            let result = self.new_inode(nw, ne, sw, se);

            if let (id, Node::Inode(mut inode)) = self.table.remove_by_left(&tree).unwrap() {
                inode.result = Some(result);
                self.table.insert(id, Node::Inode(inode));
            }
            result
        }
    }

    fn manual_evolve(&mut self, node: Id) -> Id {
        let inode = node.inode(self);
        debug_assert!(
            inode.level == 2,
            "manual evolution only at level 2 possible"
        );

        let mut all_bits: u16 = 0;
        for y in -2..2 {
            for x in -2..2 {
                all_bits = (all_bits << 1) + self.get_tree_cell(node, (x, y)) as u16;
            }
        }
        let (nw, ne, sw, se) = (
            self.one_gen(all_bits >> 5),
            self.one_gen(all_bits >> 4),
            self.one_gen(all_bits >> 1),
            self.one_gen(all_bits),
        );
        self.new_inode(nw, ne, sw, se)
    }

    fn one_gen(&mut self, mut bitmask: u16) -> Id {
        if bitmask == 0 {
            return self.new_leaf(Cell::Dead);
        }
        let center = (bitmask >> 5) & 1;
        bitmask &= 0b00000__111_0101_0111; // mask out bits we don't care about
        let neighbor_count = bitmask.count_ones();
        if neighbor_count == 3 || (neighbor_count == 2 && center != 0) {
            self.new_leaf(Cell::Alive)
        } else {
            self.new_leaf(Cell::Dead)
        }
    }

    fn centered_horizontal(&mut self, west: Id, east: Id) -> Id {
        let (west, east) = (west.inode(self), east.inode(self));
        debug_assert!(west.level == east.level, "levels must be the same");
        let (nw, ne, sw, se) = (
            west.ne.inode(self).se,
            east.nw.inode(self).sw,
            west.se.inode(self).ne,
            east.sw.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }

    fn centered_vertical(&mut self, north: Id, south: Id) -> Id {
        let (north, south) = (north.inode(self), south.inode(self));
        debug_assert!(north.level == south.level, "levels must be the same");

        let (nw, ne, sw, se) = (
            north.sw.inode(self).se,
            north.se.inode(self).sw,
            south.nw.inode(self).ne,
            south.ne.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }

    fn centered_sub(&mut self, node: Id) -> Id {
        let node = node.inode(self);

        let (nw, ne, sw, se) = (
            node.nw.inode(self).se,
            node.ne.inode(self).sw,
            node.sw.inode(self).ne,
            node.se.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se) 
    }

    fn centered_subsub(&mut self, node: Id) -> Id {
        let node = node.inode(self);
        let (nw, ne, sw, se) = (
            node.nw.inode(self).se.inode(self).se,
            node.ne.inode(self).sw.inode(self).sw,
            node.sw.inode(self).ne.inode(self).ne,
            node.se.inode(self).nw.inode(self).nw,
        );
        self.new_inode(nw, ne, sw, se)
    }
}

impl Universe {
    pub fn set_cell(&mut self, pos: impl Into<Position>, cell: Cell) {
        let pos = pos.into();

        loop {
            let level = self.root.unwrap().node(self).level();
            if pos.in_bounds(level) {
                break;
            }
            self.expand();
        }

        self.root = Some(self.set_tree_cell(self.root.unwrap(), pos, cell));
    }

    pub fn get_cell(&self, pos: impl Into<Position>) -> Cell {
        let pos = pos.into();
        let root = self.root.unwrap();
        let coord_range = root.node(self).level().coord_range();
        if coord_range.contains(&pos.x) && coord_range.contains(&pos.y) {
            self.get_tree_cell(root, pos)
        } else {
            Cell::Dead
        }
    }

    pub fn evolve(&mut self) {
        loop {
            let iroot = self.root.unwrap().inode(self);
            let (nw_pop, ne_pop, sw_pop, se_pop) = (
                iroot.nw.node(self).population(),
                iroot.ne.node(self).population(),
                iroot.sw.node(self).population(),
                iroot.se.node(self).population(),
            );

            let (nw_inner_pop, ne_inner_pop, sw_inner_pop, se_inner_pop) = (
                iroot
                    .nw
                    .inode(self)
                    .se
                    .inode(self)
                    .se
                    .node(self)
                    .population(),
                iroot
                    .ne
                    .inode(self)
                    .sw
                    .inode(self)
                    .sw
                    .node(self)
                    .population(),
                iroot
                    .sw
                    .inode(self)
                    .ne
                    .inode(self)
                    .ne
                    .node(self)
                    .population(),
                iroot
                    .se
                    .inode(self)
                    .nw
                    .inode(self)
                    .nw
                    .node(self)
                    .population(),
            );

            if self.root.unwrap().node(self).level() >= 3
                && nw_pop == nw_inner_pop
                && ne_pop == ne_inner_pop
                && sw_pop == sw_inner_pop
                && se_pop == se_inner_pop
            {
                break;
            }
            self.expand();
        }

        let root = self.root.unwrap();

        self.root = Some(self.evolve_tree(root));
        self.generation += 1;
    }
}