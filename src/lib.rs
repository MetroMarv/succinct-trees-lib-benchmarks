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
    use bv::{Bits,BitVec};
    use std::fs::File;
    use test::Bencher;

    #[bench]
    fn bench_read_bp_10000_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_bp();
        });
    }

    #[bench]
    fn bench_read_louds_10000_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_louds();
        });
    }

    #[bench]
    fn bench_read_bp_27_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_bp_nodes_27_depth_10_max_children_2.txt");
        let fileResult= File::open(path);

        let file = match fileResult {
            Ok(file) => file,
            Err(err) => panic!("My Error. {:?}", err)
        };

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_bp();
        });
    }

    #[bench]
    fn bench_read_louds_27_nodes(b: &mut Bencher) {
        let path = String::from("resources/parenthesis_louds_nodes_27_depth_10_max_children_2.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        b.iter(|| {
            let bp = parser.read_louds();
        });
    }

    fn get_read_louds_tree_27() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_27_depth_10_max_children_2.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds()
    }

    fn get_read_louds_tree_200() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_194_depth_5_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds()
    }

    fn get_read_louds_tree_1600() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_1670_depth_10_max_children_3.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds()
    }

    fn get_read_louds_tree_8000() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_7691_depth_5_max_children_10.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds()
    }

    fn get_read_louds_tree_100000() -> succinct_trees::louds::Louds{
        let path = String::from("resources/parenthesis_louds_nodes_128457_depth_10_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_louds()
    }

    fn get_read_bp_tree_100000() -> succinct_trees::bp::BalancedParenthesis{
        let path = String::from("resources/parenthesis_bp_nodes_128457_depth_10_max_children_5.txt");
        let file = File::open(path).unwrap();

        let mut parser = TreeParser::new(file);

        parser.read_bp()
    }

    #[bench]
    fn bench_louds_is_leaf_100000(b: &mut Bencher) {
        let tree = get_read_louds_tree_100000();

        b.iter(|| {
            tree.is_leaf(10000);
        })
    }

    #[bench]
    fn bench_bp_is_leaf_100000(b: &mut Bencher) {
        let tree = get_read_bp_tree_100000();

        b.iter(|| {
            tree.is_leaf(100000);
        })
    }

    #[bench]
    fn bench_louds_parent_27(b: &mut Bencher) {
        let tree = get_read_louds_tree_27();

        b.iter(|| {
            tree.parent(36);
        })
    }

    #[bench]
    fn bench_louds_parent_200(b: &mut Bencher) {
        let tree = get_read_louds_tree_200();

        for i in 8..388 {
            tree.parent(i);
            println!("Success for {}", i);
        }
    }

    #[bench]
    fn bench_louds_parent_1600(b: &mut Bencher) {
        let tree = get_read_louds_tree_1600();

        b.iter(|| {
            tree.parent(1700);
        })
    }

    #[bench]
    fn bench_louds_parent_8000(b: &mut Bencher) {
        let tree = get_read_louds_tree_8000();

        b.iter(|| {
            tree.parent(4000);
        })
    }

    #[bench]
    fn bench_louds_parent_100000(b: &mut Bencher) {
        let tree = get_read_louds_tree_100000();

        b.iter(|| {
            tree.parent(100000);
        })
    }

    #[bench]
    fn bench_bp_parent_100000(b: &mut Bencher) {
        let tree = get_read_bp_tree_100000();

        b.iter(|| {
            tree.parent(99988);
        })
    }

    #[bench]
    fn bench_louds_first_child_100000(b: &mut Bencher) {
        let tree = get_read_louds_tree_100000();

        b.iter(|| {
            tree.first_child(100000);
        })
    }

    #[bench]
    fn bench_bp_first_child_100000(b: &mut Bencher) {
        let tree = get_read_bp_tree_100000();

        b.iter(|| {
            tree.first_child(99988);
        })
    }
}
