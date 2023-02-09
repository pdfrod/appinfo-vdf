use std::ffi::CStr;
use std::io::Result;
use std::io::Write;
use super::{VDF, VDFAppNode, VDFAppNodeKind, VDFAppSection, VDFHeader};

pub fn write<S>(stream: &mut S, vdf: &VDF) -> Result<()>
where S: Write {
    write_vdf_header(stream, &vdf.header)?;
    write_vdf_sections(stream, &vdf.sections)?;
    Ok(())
}

fn write_vdf_header<S>(stream: &mut S, header: &VDFHeader) -> Result<()>
where S: Write {
    stream.write(&header.magic.to_le_bytes())?;
    stream.write(&header.version.to_le_bytes())?;
    Ok(())
}

fn write_vdf_sections<S>(stream: &mut S, sections: &[VDFAppSection]) -> Result<()>
where S: Write {
    for section in sections {
        write_vdf_section(stream, section)?;
    }
    stream.write(&[0, 0, 0, 0])?;
    Ok(())
}

fn write_vdf_section<S>(stream: &mut S, section: &VDFAppSection) -> Result<()>
where S: Write {
    stream.write(&section.app_id.to_le_bytes())?;
    let mut data = Vec::new();
    data.write(&section.info_state.to_le_bytes())?;
    data.write(&section.last_updated.to_le_bytes())?;
    data.write(&section.pics_token.to_le_bytes())?;
    data.write(&section.sha1)?; //FIXME: sha1 needs to be recomputed
    data.write(&section.binary_sha1)?; //FIXME: binary_sha1 needs to be recomputed
    data.write(&section.change_number.to_le_bytes())?;
    write_vdf_app_nodes(&mut data, &section.nodes)?;
    let data_size = data.len() as u32;
    stream.write(&data_size.to_le_bytes())?;
    stream.write(&data)?;
    Ok(())
}

fn write_vdf_app_nodes<S>(stream: &mut S, nodes: &[VDFAppNode]) -> Result<()>
where S: Write {
    for node in nodes {
        write_vdf_app_node(stream, node)?;
    }
    stream.write(&[VDFAppNodeKind::End as u8])?;
    Ok(())
}

fn write_vdf_app_node<S>(stream: &mut S, node: &VDFAppNode) -> Result<()>
where S: Write {
    match node {
        VDFAppNode::Simple{..} => write_vdf_app_node_simple(stream, node)?,
        VDFAppNode::Str{..}    => write_vdf_app_node_str(stream, node)?,
        VDFAppNode::Int{..}    => write_vdf_app_node_int(stream, node)?,
    }
    Ok(())
}

fn write_vdf_app_node_simple<S>(stream: &mut S, node: &VDFAppNode) -> Result<()>
where S: Write {
    stream.write(&[VDFAppNodeKind::Simple as u8])?;
    if let VDFAppNode::Simple{ name, children } = node {
        write_vdf_str(stream, name)?;
        write_vdf_app_nodes(stream, children)?;
    }
    Ok(())
}

fn write_vdf_app_node_str<S>(stream: &mut S, node: &VDFAppNode) -> Result<()>
where S: Write {
    stream.write(&[VDFAppNodeKind::Str as u8])?;
    if let VDFAppNode::Str{ name, value } = node {
        write_vdf_str(stream, name)?;
        write_vdf_str(stream, value)?;
    }
    Ok(())
}

fn write_vdf_app_node_int<S>(stream: &mut S, node: &VDFAppNode) -> Result<()>
where S: Write {
    stream.write(&[VDFAppNodeKind::Int as u8])?;
    if let VDFAppNode::Int{ name, value } = node {
        write_vdf_str(stream, name)?;
        stream.write(&value.to_le_bytes())?;
    }
    Ok(())
}

fn write_vdf_str<S>(stream: &mut S, string: &CStr) -> Result<()>
where S: Write {
    stream.write(string.to_bytes_with_nul())?;
    Ok(())
}
