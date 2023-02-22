use comrak::markdown_to_html;

pub(crate) fn to_html(markdown: &str) -> String {
    markdown_to_html(markdown, &Default::default())
}
