//! Minimal EPUB metadata extractor:
//! - open ZIP
//! - read META-INF/container.xml → locate OPF
//! - parse OPF for dc:title, dc:creator, dc:description, dc:publisher, dc:date
//! (Best effort; chapters left empty for now.)

use crate::model::EpubMeta;
use anyhow::{Context, Result, anyhow};
use roxmltree::Document;
use std::io::Read;
use std::path::Path;
use std::{collections::BTreeMap, fs::File};
use zip::read::ZipArchive;

pub fn extract_epub_metadata(path: &Path) -> Result<EpubMeta> {
    let file = File::open(path).with_context(|| format!("open epub: {}", path.display()))?;
    let mut zip = ZipArchive::new(file).with_context(|| "open zip archive")?;

    // 1) container.xml
    let mut container_xml = String::new();
    zip.by_name("META-INF/container.xml")
        .map_err(|_| anyhow!("container.xml not found"))?
        .read_to_string(&mut container_xml)?;
    let doc = Document::parse(&container_xml)?;
    let rootfile = doc
        .descendants()
        .find(|n| n.has_tag_name("rootfile"))
        .and_then(|n| n.attribute("full-path"))
        .ok_or_else(|| anyhow!("rootfile@full-path missing in container.xml"))?
        .to_string();

    // 2) OPF
    let mut opf_xml = String::new();
    zip.by_name(&rootfile)
        .map_err(|_| anyhow!("OPF not found at {rootfile}"))?
        .read_to_string(&mut opf_xml)?;
    let opf = Document::parse(&opf_xml)?;

    let ns_dc = "http://purl.org/dc/elements/1.1/";

    let text_of = |tag: &str| {
        opf.descendants()
            .find(|n| {
                (n.tag_name().name() == tag && n.tag_name().namespace() == Some(ns_dc))
                    || n.tag_name().name() == tag // tolerate missing namespace
            })
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
    };

    let title = text_of("title");
    let author = text_of("creator");
    let description = text_of("description");
    let publisher = text_of("publisher");
    let publish_date = text_of("date");

    // You can enrich other_metadata with language, identifier, subject, etc.
    let mut other = BTreeMap::new();
    if let Some(lang) = text_of("language") {
        other.insert("language".into(), lang);
    }
    if let Some(ident) = text_of("identifier") {
        other.insert("identifier".into(), ident);
    }

    Ok(EpubMeta {
        title,
        author,
        description,
        chapters: Vec::new(), // TODO: parse NCX/nav for chapter titles
        publish_date,
        publisher,
        other_metadata: other,
    })
}
