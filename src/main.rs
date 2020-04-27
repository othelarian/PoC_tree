use std::fmt;

// TREE TRAIT ###################################

trait Tree: fmt::Display {}

// LEAVES #######################################

// Leaf ---------------------

struct EmptyLeaf;

impl fmt::Display for EmptyLeaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An empty leaf")
    }
}

impl Tree for EmptyLeaf {}

// IntLeaf ------------------

struct IntLeaf(isize);

impl fmt::Display for IntLeaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A leaf who contain an integer: {}", self.0)
    }
}

impl Tree for IntLeaf{}

// StringLeaf --------------

struct StringLeaf<'a>(&'a str);

impl<'a> fmt::Display for StringLeaf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A leaf containing the following text: {}", self.0)
    }
}

impl<'a> Tree for StringLeaf<'a>{}

// TupleLeaf ----------------

struct TupleLeaf<A, B>(A, B)
where A: fmt::Display, B: fmt::Display;

impl<A, B> fmt::Display for TupleLeaf<A, B>
where A: fmt::Display, B: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A leaf with two values in it: {} and {}", self.0, self.1)
    }
}

impl<A, B> Tree for TupleLeaf<A, B>
where A: fmt::Display, B: fmt::Display {}

// NODES ########################################

// BinaryNode ---------------

struct BinaryNode<L, R>
where L: Tree, R: Tree {
    left: L,
    right: R
}

impl<L, R> fmt::Display for BinaryNode<L, R>
where L: Tree, R: Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Binary node:\n  left: {},\n  right: {}", self.left, self.right)
    }
}

impl<L, R> Tree for BinaryNode<L, R>
where L: Tree, R: Tree {}

// TriNode ------------------

struct TriNode<L, C, R>
where L: Tree, C: Tree, R: Tree {
    left: L,
    center: C,
    right: R
}

impl<L, C, R> fmt::Display for TriNode<L, C, R>
where L: Tree, C: Tree, R: Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tri node:\n  left: {},\n  center: {},\n  right: {}",
            self.left,
            self.center,
            self.right
        )
    }
}

impl<L, C, R> Tree for TriNode<L, C, R>
where L: Tree, C: Tree, R: Tree {}

// MAIN #########################################

fn main() {
    let leaf_dl = TupleLeaf(645, "leaf_df");
    println!("leaf for the left of a binary node:\n{}\n", leaf_dl);
    let leaf_dr = StringLeaf("leaf_dr");
    println!("leaf for th right of a binary node:\n{}\n", leaf_dr);
    let node_d = BinaryNode {left: leaf_dl, right: leaf_dr};
    println!("a binary node:\n{}\n\n", node_d);
    println!("--------------------------\n");
    let leaf_tl = EmptyLeaf;
    println!("leaf for the top left side:\n{}\n", leaf_tl);
    let leaf_tc = IntLeaf(13);
    println!("leaf for the top center:\n{}\n", leaf_tc);
    let trinode = TriNode {
        left: leaf_tl,
        center: leaf_tc,
        right: node_d
    };
    println!("\nA big tree:\n{}", trinode);
}
