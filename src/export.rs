use crate::pdf::PdfResult;
use rust_xlsxwriter::*;

pub fn generate_csv(results: &[PdfResult]) -> String {
    let mut wtr = csv::Writer::from_writer(Vec::new());

    let _ = wtr.write_record([
        "Archivo",
        "Ruta",
        "Título",
        "Autor",
        "Creador",
        "Productor",
        "Fecha de creación",
        "Fecha de modificación",
        "SHA-256",
        "Resultado",
        "Error",
    ]);

    for r in results {
        let meta = r.metadata.as_ref();
        let _ = wtr.write_record([
            r.filename.clone(),
            r.relative_path.clone(),
            meta.and_then(|m| m.title.clone()).unwrap_or_default(),
            meta.and_then(|m| m.author.clone()).unwrap_or_default(),
            meta.and_then(|m| m.creator.clone()).unwrap_or_default(),
            meta.and_then(|m| m.producer.clone()).unwrap_or_default(),
            meta.and_then(|m| m.creation_date.clone())
                .unwrap_or_default(),
            meta.and_then(|m| m.mod_date.clone()).unwrap_or_default(),
            r.sha256.clone().unwrap_or_default(),
            if r.status == crate::pdf::ResultStatus::Ok {
                "OK".to_string()
            } else {
                "ERROR".to_string()
            },
            r.error.clone().unwrap_or_default(),
        ]);
    }

    String::from_utf8(wtr.into_inner().unwrap()).unwrap()
}

pub fn generate_xlsx(results: &[PdfResult]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let headers = [
        "Archivo",
        "Ruta",
        "Título",
        "Autor",
        "Creador",
        "Productor",
        "Fecha de creación",
        "Fecha de modificación",
        "SHA-256",
        "Resultado",
        "Error",
    ];

    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x4A90D9))
        .set_font_color(Color::White);

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_with_format(0, col as u16, *header, &header_format)?;
    }

    let ok_format = Format::new()
        .set_font_color(Color::RGB(0x2E7D32))
        .set_bold();
    let err_format = Format::new()
        .set_font_color(Color::RGB(0xC62828))
        .set_bold();

    for (row, r) in results.iter().enumerate() {
        let row_idx = (row + 1) as u32;
        let meta = r.metadata.as_ref();

        worksheet.write(row_idx, 0, &r.filename)?;
        worksheet.write(row_idx, 1, &r.relative_path)?;
        worksheet.write(
            row_idx,
            2,
            meta.and_then(|m| m.title.as_deref()).unwrap_or(""),
        )?;
        worksheet.write(
            row_idx,
            3,
            meta.and_then(|m| m.author.as_deref()).unwrap_or(""),
        )?;
        worksheet.write(
            row_idx,
            4,
            meta.and_then(|m| m.creator.as_deref()).unwrap_or(""),
        )?;
        worksheet.write(
            row_idx,
            5,
            meta.and_then(|m| m.producer.as_deref()).unwrap_or(""),
        )?;
        worksheet.write(
            row_idx,
            6,
            meta.and_then(|m| m.creation_date.as_deref()).unwrap_or(""),
        )?;
        worksheet.write(
            row_idx,
            7,
            meta.and_then(|m| m.mod_date.as_deref()).unwrap_or(""),
        )?;
        worksheet.write(row_idx, 8, r.sha256.as_deref().unwrap_or(""))?;

        let (label, fmt) = if r.status == crate::pdf::ResultStatus::Ok {
            ("OK", &ok_format)
        } else {
            ("ERROR", &err_format)
        };
        worksheet.write_with_format(row_idx, 9, label, fmt)?;
        worksheet.write(row_idx, 10, r.error.as_deref().unwrap_or(""))?;
    }

    worksheet.set_column_width(0, 35)?;
    worksheet.set_column_width(1, 45)?;
    worksheet.set_column_width(2, 30)?;
    worksheet.set_column_width(3, 20)?;
    worksheet.set_column_width(4, 25)?;
    worksheet.set_column_width(5, 25)?;
    worksheet.set_column_width(6, 22)?;
    worksheet.set_column_width(7, 22)?;
    worksheet.set_column_width(8, 64)?;
    worksheet.set_column_width(9, 10)?;
    worksheet.set_column_width(10, 40)?;

    Ok(workbook.save_to_buffer()?)
}
