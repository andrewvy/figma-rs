use std::fs;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

use figma_api::file::{File, Node};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    token: Option<String>,

    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

pub struct JavascriptWriter<W: Write> {
    writer: W,
}

impl<W: Write> JavascriptWriter<W> {
    pub fn new(writer: W) -> Self {
        JavascriptWriter { writer }
    }

    fn raw_write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let written = self.writer.write(data)?;

        Ok(written)
    }

    fn write(&mut self, data: &str) -> std::io::Result<usize> {
        let mut count = 0;

        if !data.is_empty() {
            count += self.raw_write(data.as_bytes())?;
        }

        Ok(count)
    }

    fn write_line(&mut self) -> std::io::Result<usize> {
        self.raw_write("\n".as_bytes())
    }
}

fn render<W: Write>(file: &File, writer: &mut JavascriptWriter<W>) -> std::io::Result<usize> {
    let mut bytes = 0;

    match &file.document {
        figma_api::file::Node::Document { children, .. } => {
            for child in children {
                bytes += render_node(child, writer)?;
            }

            Ok(bytes)
        }
        _ => panic!("File does not contain a root document node"),
    }
}

fn render_node<W: Write>(node: &Node, writer: &mut JavascriptWriter<W>) -> std::io::Result<usize> {
    let empty = vec![];
    let (mut bytes, children) = match node {
        Node::Document { .. } => (0, &empty),
        Node::Canvas { node, children, .. } => (writer.write(&node.name)?, children),
        Node::Frame { node, children, .. } => (writer.write(&node.name)?, children),
        Node::Group { node, children, .. } => (writer.write(&node.name)?, children),
        Node::Vector { node, .. } => (writer.write(&node.name)?, &empty),
        Node::BooleanOperation { node, children, .. } => (writer.write(&node.name)?, children),
        Node::Star { node, .. } => (writer.write(&node.name)?, &empty),
        Node::Line { node, .. } => (writer.write(&node.name)?, &empty),
        Node::Ellipse { node, .. } => (writer.write(&node.name)?, &empty),
        Node::RegularPolygon { node, .. } => (writer.write(&node.name)?, &empty),
        Node::Rectangle { node, .. } => (writer.write(&node.name)?, &empty),
        Node::Text { node, .. } => (writer.write(&node.name)?, &empty),
        Node::Slice { node, .. } => (writer.write(&node.name)?, &empty),
        Node::Component { node, children, .. } => (writer.write(&node.name)?, children),
        Node::ComponentSet { node, children, .. } => (writer.write(&node.name)?, children),
        Node::Instance { node, children, .. } => (writer.write(&node.name)?, children),
    };

    bytes += writer.write_line()?;

    for child in children.into_iter() {
        bytes += render_node(child, writer)?;
        bytes += writer.write_line()?;
    }

    Ok(bytes)
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let contents = fs::read(opt.file)?;
    let contents = String::from_utf8(contents).expect("Invalid UTF-8");

    let document: File = serde_json::from_str(&contents)?;
    let output = std::fs::File::create(opt.output.clone())?;
    let mut js_writer = JavascriptWriter::new(output);

    let bytes = render(&document, &mut js_writer)?;

    println!("Wrote {} bytes to {:?}.", bytes, &opt.output);

    Ok(())
}
