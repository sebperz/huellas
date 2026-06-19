use lopdf::{Document, Object, ObjectId};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub mod_date: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PdfResult {
    pub filename: String,
    pub relative_path: String,
    pub metadata: Option<PdfMetadata>,
    pub sha256: Option<String>,
    pub status: ResultStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResultStatus {
    Ok,
    Error,
}

pub fn analyze_pdf(data: &[u8], filename: &str, relative_path: &str) -> PdfResult {
    let sha256 = calculate_sha256(data);

    match Document::load_mem(data) {
        Ok(doc) => {
            let metadata = extract_metadata(&doc);
            let all_fields_present = metadata
                .title
                .as_ref()
                .map_or(false, |v| !v.trim().is_empty())
                && metadata
                    .author
                    .as_ref()
                    .map_or(false, |v| !v.trim().is_empty())
                && metadata
                    .creator
                    .as_ref()
                    .map_or(false, |v| !v.trim().is_empty())
                && metadata
                    .producer
                    .as_ref()
                    .map_or(false, |v| !v.trim().is_empty())
                && metadata
                    .creation_date
                    .as_ref()
                    .map_or(false, |v| !v.trim().is_empty())
                && metadata
                    .mod_date
                    .as_ref()
                    .map_or(false, |v| !v.trim().is_empty());

            PdfResult {
                filename: filename.to_string(),
                relative_path: relative_path.to_string(),
                metadata: Some(metadata),
                sha256: Some(sha256),
                status: if all_fields_present {
                    ResultStatus::Ok
                } else {
                    ResultStatus::Error
                },
                error: if all_fields_present {
                    None
                } else {
                    Some("Campos obligatorios faltantes o vacíos".to_string())
                },
            }
        }
        Err(e) => {
            let error_msg = if format!("{:?}", e).contains("encrypted") {
                "PDF encriptado — no se puede leer".to_string()
            } else {
                format!("PDF corrupto o inválido: {}", e)
            };

            PdfResult {
                filename: filename.to_string(),
                relative_path: relative_path.to_string(),
                metadata: None,
                sha256: Some(sha256),
                status: ResultStatus::Error,
                error: Some(error_msg),
            }
        }
    }
}

fn extract_metadata(doc: &Document) -> PdfMetadata {
    let xmp = extract_xmp_metadata(doc);
    let info = extract_info_metadata(doc);

    PdfMetadata {
        title: xmp.title.or(info.title),
        author: xmp.author.or(info.author),
        creator: xmp.creator.or(info.creator),
        producer: xmp.producer.or(info.producer),
        creation_date: xmp.creation_date.or(info.creation_date),
        mod_date: xmp.mod_date.or(info.mod_date),
    }
}

fn object_to_string(obj: &Object) -> Option<String> {
    match obj {
        Object::String(bytes, _) => String::from_utf8(bytes.clone()).ok(),
        Object::Name(name) => String::from_utf8(name.clone()).ok(),
        _ => None,
    }
}

fn to_ref(obj: &Object) -> Option<ObjectId> {
    match obj {
        Object::Reference(id) => Some(*id),
        _ => None,
    }
}

fn get_info_value(doc: &Document, key: &[u8]) -> Option<String> {
    let info_obj_ref = match doc.trailer.get(b"Info") {
        Ok(obj) => obj,
        Err(_) => return None,
    };
    let info_ref = to_ref(info_obj_ref)?;
    let info_obj = match doc.get_object(info_ref) {
        Ok(obj) => obj,
        Err(_) => return None,
    };
    let dict = match info_obj.as_dict() {
        Ok(d) => d,
        Err(_) => return None,
    };
    let value = match dict.get(key) {
        Ok(v) => v,
        Err(_) => return None,
    };
    object_to_string(value)
}

fn get_info_date(doc: &Document, key: &[u8]) -> Option<String> {
    get_info_value(doc, key).map(|s| s.trim_start_matches("D:").trim().to_string())
}

fn extract_info_metadata(doc: &Document) -> PdfMetadata {
    PdfMetadata {
        title: get_info_value(doc, b"Title"),
        author: get_info_value(doc, b"Author"),
        creator: get_info_value(doc, b"Creator"),
        producer: get_info_value(doc, b"Producer"),
        creation_date: get_info_date(doc, b"CreationDate"),
        mod_date: get_info_date(doc, b"ModDate"),
    }
}

fn extract_xmp_metadata(doc: &Document) -> PdfMetadata {
    let empty = PdfMetadata {
        title: None,
        author: None,
        creator: None,
        producer: None,
        creation_date: None,
        mod_date: None,
    };

    let root_obj = match doc.trailer.get(b"Root") {
        Ok(obj) => obj,
        Err(_) => return empty,
    };
    let root_ref = match to_ref(root_obj) {
        Some(r) => r,
        None => return empty,
    };

    let root = match doc.get_object(root_ref) {
        Ok(obj) => obj,
        Err(_) => return empty,
    };

    let root_dict = match root.as_dict() {
        Ok(d) => d,
        Err(_) => return empty,
    };

    let meta_obj_ref = match root_dict.get(b"Metadata") {
        Ok(obj) => obj,
        Err(_) => return empty,
    };
    let meta_ref = match to_ref(meta_obj_ref) {
        Some(r) => r,
        None => return empty,
    };

    let meta_obj = match doc.get_object(meta_ref) {
        Ok(obj) => obj,
        Err(_) => return empty,
    };

    let stream = match meta_obj.as_stream() {
        Ok(s) => s,
        Err(_) => return empty,
    };

    let xmp_bytes = match stream.decompressed_content() {
        Ok(b) => b,
        Err(_) => return empty,
    };

    let xmp_str = match std::str::from_utf8(&xmp_bytes) {
        Ok(s) => s,
        Err(_) => return empty,
    };

    PdfMetadata {
        title: extract_xmp_field(xmp_str, "dc:title"),
        author: extract_xmp_field(xmp_str, "dc:creator"),
        creator: extract_xmp_field(xmp_str, "xmp:CreatorTool"),
        producer: extract_xmp_field(xmp_str, "pdf:Producer"),
        creation_date: extract_xmp_field(xmp_str, "xmp:CreateDate"),
        mod_date: extract_xmp_field(xmp_str, "xmp:ModifyDate"),
    }
}

fn extract_xmp_field(xml: &str, field: &str) -> Option<String> {
    let tag = field.split(':').last()?;

    let patterns: Vec<String> = vec![
        format!("<{}>", tag),
        format!("<{} ", tag),
        format!("<{}:", field),
        format!("<{} ", field),
    ];

    for pattern in &patterns {
        if let Some(start) = xml.find(pattern.as_str()) {
            let remaining = &xml[start..];
            let close = remaining.find('>')?;
            let content_start = close + 1;
            let content_remaining = &remaining[content_start..];
            let end_tag = format!("</{}>", tag);
            let end_tag_alt = format!("</{}:", field);

            let end = content_remaining
                .find(&end_tag)
                .or_else(|| content_remaining.find(&end_tag_alt))?;

            let value = content_remaining[..end].trim().to_string();

            if !value.is_empty()
                && value != "rdf:Bag"
                && !value.contains("rdf:li")
                && !value.starts_with('<')
            {
                return Some(value);
            }

            if let Some(li_start) = value.find("<rdf:li>") {
                let li_content = &value[li_start + 8..];
                if let Some(li_end) = li_content.find("</rdf:li>") {
                    let li_value = li_content[..li_end].trim().to_string();
                    if !li_value.is_empty() {
                        return Some(li_value);
                    }
                }
            }
        }
    }

    None
}

fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
