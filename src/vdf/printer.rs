use super::{VDF, VDFAppNode, VDFAppSection};

pub fn print(vdf: &VDF) {
    println!("# VDFHeader");
    println!("magic: {}", vdf.header.magic);
    println!("version: {}\n", vdf.header.version);

    for section in &vdf.sections {
        print_app_section(&section);
        println!("\n");
    }
}

fn print_app_section(section: &VDFAppSection) {
    println!("# VDFAppSection");
    println!("app_id: {}", section.app_id);
    println!("data_size: {}", section.data_size);
    println!("info_state: {}", section.info_state);
    println!("last_updated: {}", section.last_updated);
    println!("pics_token: {}", section.pics_token);
    println!("sha1: {:?}", section.sha1);
    println!("binary_sha1: {:?}", section.binary_sha1);
    println!("change_number: {}", section.change_number);
    print_app_nodes(&section.nodes, 0);
}

fn print_app_nodes(nodes: &[VDFAppNode], level: usize) {
    println!("{{");
    for (i, node) in nodes.iter().enumerate() {
        print_app_node(node, level + 1);
        let sep = if i == nodes.len() - 1 { "" } else { "," };
        println!("{}", sep);
    }
    print!("{:width$}}}", "", width = level * 2);
}

fn print_app_node(node: &VDFAppNode, level: usize) {
    match node {
        VDFAppNode::Simple{ name, children } => {
            print!("{:width$}{:?}: ", "", name, width = level * 2);
            print_app_nodes(children, level);
        },
        VDFAppNode::Str{ name, value } => {
            print!("{:width$}{:?}: {:?}", "", name, value, width = level * 2);
        }
        VDFAppNode::Int{ name, value } => {
            print!("{:width$}{:?}: {}", "", name, value, width = level * 2);
        }
    }
}
