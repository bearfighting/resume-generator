//! WordprocessingML primitives. Plain text always belongs to a run; paragraphs
//! accept inline nodes and tables are followed by a separating paragraph.
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn text(text: &str) -> String {
    format!("<w:t xml:space=\"preserve\">{}</w:t>", escape(text))
}

pub fn run(text_value: &str, properties: &str) -> String {
    format!("<w:r><w:rPr>{properties}</w:rPr>{}</w:r>", text(text_value))
}

pub fn paragraph(style: &str, properties: &str, inline: &str) -> String {
    format!("<w:p><w:pPr><w:pStyle w:val=\"{style}\"/>{properties}</w:pPr>{inline}</w:p>")
}

pub fn cell(width: u32, properties: &str, blocks: &str) -> String {
    format!(
        "<w:tc><w:tcPr><w:tcW w:w=\"{width}\" w:type=\"dxa\"/>{properties}</w:tcPr>{blocks}</w:tc>"
    )
}

pub fn row(cells: &[String]) -> String {
    format!(
        "<w:tr><w:trPr><w:cantSplit/></w:trPr>{}</w:tr>",
        cells.join("")
    )
}

pub fn table(width: u32, columns: &[u32], properties: &str, rows: &[String]) -> String {
    let grid: String = columns
        .iter()
        .map(|width| format!("<w:gridCol w:w=\"{width}\"/>"))
        .collect();
    format!("<w:tbl><w:tblPr><w:tblW w:w=\"{width}\" w:type=\"dxa\"/><w:tblLayout w:type=\"fixed\"/>{properties}</w:tblPr><w:tblGrid>{grid}</w:tblGrid>{}</w:tbl>", rows.join(""))
}

pub fn separate_tables(body: &str) -> String {
    body.replace("</w:tbl>", "</w:tbl><w:p><w:pPr><w:spacing w:before=\"0\" w:after=\"80\" w:line=\"20\" w:lineRule=\"exact\"/></w:pPr><w:r><w:rPr><w:sz w:val=\"2\"/></w:rPr><w:t/></w:r></w:p>")
}
