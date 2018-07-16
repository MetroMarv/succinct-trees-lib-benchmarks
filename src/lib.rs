#![feature(test)]
extern crate test;
extern crate succinct_trees_lib;
#[macro_use]
extern crate bv;

#[cfg(test)]
mod tests {
    use succinct_trees_lib::succinct_trees::bp::BalancedParenthesis;
    use succinct_trees_lib::succinct_trees;
    use succinct_trees_lib::succinct_trees::parser::TreeParser;
    use succinct_trees_lib::succinct_trees::SuccinctTreeFunctions;
    use std::fs::File;
    use test::Bencher;



    /**
     * Benchmarks with approx. 1600 nodes tree
     */

    fn get_read_louds_tree_1600() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_1670_depth_10_max_children_3.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds()
    }

    fn get_read_bp_tree_1600() -> succinct_trees::bp::BalancedParenthesis{
        let path = String::from("resources/parenthesis_bp_nodes_1670_depth_10_max_children_3.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_bp()
    }

    #[bench]
    fn bench_1600_louds_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_1670_depth_10_max_children_3.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_louds();
        });
    }

    #[bench]
    fn bench_1600_bp_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_1670_depth_10_max_children_3.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_bp();
        });
    }

    #[bench]
    fn bench_1600_louds_is_leaf(b: &mut Bencher) {
        let tree = get_read_louds_tree_1600();

        b.iter(|| {
            tree.is_leaf(1600);
        })
    }

    #[bench]
    fn bench_1600_bp_is_leaf(b: &mut Bencher) {
        let tree = get_read_bp_tree_1600();

        b.iter(|| {
            tree.is_leaf(1600);
        })
    }

    #[bench]
    fn bench_1600_louds_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_1600();

        b.iter(|| {
            tree.parent(1600);
        })
    }

    #[bench]
    fn bench_1600_bp_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_1600();

        b.iter(|| {
            tree.parent(1600);
        })
    }

    #[bench]
    fn bench_1600_louds_first_child(b: &mut Bencher) {
        let tree = get_read_louds_tree_1600();

        b.iter(|| {
            tree.first_child(1600);
        })
    }

    #[bench]
    fn bench_1600_bp_first_child(b: &mut Bencher) {
        let tree = get_read_bp_tree_1600();

        b.iter(|| {
            tree.first_child(1600);
        })
    }

    #[bench]
    fn bench_1600_louds_next_sibling(b: &mut Bencher) {
        let tree = get_read_louds_tree_1600();

        b.iter(|| {
            tree.next_sibling(1600);
        })
    }

    #[bench]
    fn bench_1600_bp_next_sibling(b: &mut Bencher) {
        let tree = get_read_bp_tree_1600();

        b.iter(|| {
            tree.next_sibling(1600);
        })
    }

    /**
     * Benchmarks with approx. 8000 nodes tree
     */


    fn get_read_louds_tree_8000() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds_blksize(256)
    }

    fn get_read_bp_tree_8000() -> succinct_trees::bp::BalancedParenthesis{
        let path = String::from("resources/parenthesis_bp_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_bp()
    }

    #[bench]
    fn bench_8000_louds_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_louds();
        });
    }

    #[bench]
    fn bench_8000_bp_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_bp();
        });
    }

    #[bench]
    fn bench_8000_louds_is_leaf(b: &mut Bencher) {
        let tree = get_read_louds_tree_8000();

        b.iter(|| {
            tree.is_leaf(8000);
        })
    }

    #[bench]
    fn bench_8000_bp_is_leaf(b: &mut Bencher) {
        let tree = get_read_bp_tree_8000();

        b.iter(|| {
            tree.is_leaf(8000);
        })
    }

    #[bench]
    fn bench_8000_louds_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_8000();

        b.iter(|| {
            tree.parent(4000);
        })
    }

    #[bench]
    fn bench_8000_bp_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_8000();

        b.iter(|| {
            tree.parent(4000);
        })
    }

    #[bench]
    fn bench_8000_louds_first_child(b: &mut Bencher) {
        let tree = get_read_louds_tree_8000();

        b.iter(|| {
            tree.first_child(8000);
        })
    }

    #[bench]
    fn bench_8000_bp_first_child(b: &mut Bencher) {
        let tree = get_read_bp_tree_8000();

        b.iter(|| {
            tree.first_child(8000);
        })
    }

    #[bench]
    fn bench_8000_louds_next_sibling(b: &mut Bencher) {
        let tree = get_read_louds_tree_8000();

        b.iter(|| {
            tree.next_sibling(8000);
        })
    }

    #[bench]
    fn bench_8000_bp_next_sibling(b: &mut Bencher) {
        let tree = get_read_bp_tree_8000();

        b.iter(|| {
            tree.next_sibling(8000);
        })
    }

    /**
     * Benchmarks with approx. 65000 nodes tree
     */

    fn get_read_louds_tree_65000() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_65726_depth_7_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds_blksize(2048)
    }

    fn get_read_bp_tree_65000() -> succinct_trees::bp::BalancedParenthesis{
        let path = String::from("resources/parenthesis_bp_nodes_65726_depth_7_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_bp()
    }

    #[bench]
    fn bench_65000_louds_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_65726_depth_7_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_louds();
        });
    }

    #[bench]
    fn bench_65000_bp_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_65726_depth_7_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_bp();
        });
    }

    #[bench]
    fn bench_65000_louds_is_leaf(b: &mut Bencher) {
        let tree = get_read_louds_tree_65000();

        b.iter(|| {
            tree.is_leaf(65000);
        })
    }

    #[bench]
    fn bench_65000_bp_is_leaf(b: &mut Bencher) {
        let tree = get_read_bp_tree_65000();

        b.iter(|| {
            tree.is_leaf(65000);
        })
    }

    #[bench]
    fn bench_65000_louds_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_65000();

        b.iter(|| {
            tree.parent(65000);
        })
    }

    #[bench]
    fn bench_65000_bp_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_65000();

        b.iter(|| {
            tree.parent(65000);
        })
    }

    #[bench]
    fn bench_65000_louds_first_child(b: &mut Bencher) {
        let tree = get_read_louds_tree_65000();

        b.iter(|| {
            tree.first_child(65000);
        })
    }

    #[bench]
    fn bench_65000_bp_first_child(b: &mut Bencher) {
        let tree = get_read_bp_tree_65000();

        b.iter(|| {
            tree.first_child(65000);
        })
    }

    #[bench]
    fn bench_65000_louds_next_sibling(b: &mut Bencher) {
        let tree = get_read_louds_tree_65000();

        b.iter(|| {
            tree.next_sibling(65000);
        })
    }

    #[bench]
    fn bench_65000_bp_next_sibling(b: &mut Bencher) {
        let tree = get_read_bp_tree_65000();

        b.iter(|| {
            tree.next_sibling(65000);
        })
    }

    /**
     * Benchmarks with 100.000 nodes
     */

    fn get_read_louds_tree_120000() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_128457_depth_10_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds_blksize(4096)
    }



    fn get_read_bp_tree_120000() -> succinct_trees::bp::BalancedParenthesis{
        let path = String::from("resources/parenthesis_bp_nodes_128457_depth_10_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_bp()
    }

    #[bench]
    fn bench_120000_louds_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_128457_depth_10_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_louds();
        });
    }

    #[bench]
    fn bench_120000_bp_read(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_128457_depth_10_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            parser.read_bp();
        });
    }

    #[bench]
    fn bench_120000_louds_is_leaf(b: &mut Bencher) {
        let tree = get_read_louds_tree_120000();

        b.iter(|| {
            tree.is_leaf(120000);
        })
    }

    #[bench]
    fn bench_120000_bp_is_leaf(b: &mut Bencher) {
        let tree = get_read_bp_tree_120000();

        b.iter(|| {
            tree.is_leaf(120000);
        })
    }

    #[bench]
    fn bench_120000_louds_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_120000();

        b.iter(|| {
            tree.parent(120000);
        })
    }

    #[bench]
    fn bench_120000_bp_parent(b: &mut Bencher) {
        let tree = get_read_louds_tree_120000();

        b.iter(|| {
            tree.parent(120000);
        })
    }

    #[bench]
    fn bench_120000_louds_first_child(b: &mut Bencher) {
        let tree = get_read_louds_tree_120000();

        b.iter(|| {
            tree.first_child(118000);
        })
    }

    #[bench]
    fn bench_120000_bp_first_child(b: &mut Bencher) {
        let tree = get_read_bp_tree_120000();

        b.iter(|| {
            tree.first_child(120000);
        })
    }

    #[bench]
    fn bench_120000_louds_next_sibling(b: &mut Bencher) {
        let tree = get_read_louds_tree_120000();

        b.iter(|| {
            tree.next_sibling(120000);
        })
    }

    #[bench]
    fn bench_120000_bp_next_sibling(b: &mut Bencher) {
        let tree = get_read_bp_tree_120000();

        b.iter(|| {
            tree.next_sibling(120000);
        })
    }
}
