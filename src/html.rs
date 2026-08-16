const STYLES: &str = r#"
* {
    box-sizing: border-box;
}
body {
    color: #222;
    display: flex;
    justify-content: center;
    line-height: 1.3;
    margin: 0 16px;
}
main {
    padding: 0 32px;
    width: 710px;
}
a {
    color: rgb(237, 26, 255);
    font: inherit;
}
a:visited {
    color: rgb(237, 26, 255);
}
.theme--dark a {
    color: rgb(242, 85, 255);
}
.theme--dark a:visited {
    color: rgb(242, 85, 255);
}
strong {
    font-weight: bold;
}
code {
    background: #f0e9ec;
    border-radius: 4px;
    color: #004297;
    font-size: 14px;
    padding: 2px;
}
blockquote {
    border-left: 4px solid #aaa;
    color: #444;
    margin: 0;
    padding-left: 20px;
}
.theme--dark .md blockquote {
    border-color: #aaa;
    color: #ccc;
}
hr {
    border-bottom: none;
    border-color: #aeaeae;
    margin: 2rem 0;
}
.toolbar {
    display: flex;
    justify-content: flex-end;
    padding: 4px 0;
}
@media print {
    .toolbar {
        display: none;
    }
}
.theme--light {
    color: #333333;
    background-color: #efefef;
}
.theme--dark {
    background-color: #223;
    color: #dddddd;
}
.font--sans {
    font-family: 'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode', Geneva, Verdana, sans-serif;
}
.font--serif {
    font-family: Georgia, 'Times New Roman', Times, serif;
}
.font--mono {
    font-family: "Roboto Mono", monospace;
    font-optical-sizing: auto;
    font-weight: 400;
    font-style: normal;
}
.md .anchor::before {
    position: absolute;
    visibility: hidden;
}
.md h1:hover .anchor::before,
.md h2:hover .anchor::before,
.md h3:hover .anchor::before,
.md h4:hover .anchor::before,
.md h5:hover .anchor::before,
.md h6:hover .anchor::before {
    visibility: visible;
    content: '#';
    left: -32px;
    width: 32px;
}

.md h1,
.md h2,
.md h3,
.md h4,
.md h5,
.md h6,
.md p {
    margin: 20px 0;
    position: relative;
    padding-left: -20px;
}
.md p img {
    width: 100%;
}
.md hr {
    border: none;
    border-bottom: 1px solid #ccc;
}
.md a {
    text-decoration: none;
}
.md blockquote {
    border-left: 4px solid #ccc;
    border-radius: 2px;
    color: #555;
    margin: 16px 4px;
    padding: 0 16px;
}
.md code {
    background-color: #eee;
    border-radius: 2px;
    padding: 2px;
}
.md li {
    margin: 4px 0;
}
@media print {
  .toolbar {
    display: none;
  }
}
@media (prefers-color-scheme: dark) {
  .theme--system {
    background-color: #223;
    color: #dddddd;
  }
  .theme--system a {
      color: rgb(242, 85, 255);
  }
  .theme--system a:visited {
      color: rgb(242, 85, 255);
  }
  .theme--system .md blockquote {
      border-color: #aaa;
      color: #ccc;
  }
}
"#;

const SCRIPT: &str = r#"

const DEFAULT_THEME = 'system';
const THEME_OPTIONS = ['system', 'light', 'dark'];

const DEFAULT_FONT = 'sans';
const DEFAULT_FONT_SIZE = 16;
const FONT_OPTIONS = ['sans', 'serif', 'mono'];

const themeSelect = document.getElementById('theme-select');
const fontSelect = document.getElementById('font-select');
const fontSizeInput = document.getElementById('font-size-input');

themeSelect.addEventListener('change', onChangeThemeSelect);
fontSelect.addEventListener('change', onChangeFontSelect);
fontSizeInput.addEventListener('input', onInputFontSize);

onPageLoad();

function onPageLoad() {

    document.querySelectorAll('a').forEach(anchorElement => {
        if (anchorElement.getAttribute('href').startsWith('#')) {
            return;
        }
        anchorElement.setAttribute('target', '_blank');
        anchorElement.setAttribute('rel', 'noopener noreferrer');
    });

    initializeTheme();
    initializeFont();
    initializeFontSize();
}

function initializeTheme() {
    let savedTheme = getSavedTheme();
    if (!savedTheme) {
        setSavedTheme(DEFAULT_THEME);
        savedTheme = getSavedTheme();
    }
    themeSelect.value = savedTheme;
    setDocumentTheme(savedTheme);
}

function getSavedTheme() {
    return localStorage.getItem('md_notes_theme');
}

function setSavedTheme(themeName) {
    localStorage.setItem('md_notes_theme', themeName);
}

function setDocumentTheme(themeName) {
    THEME_OPTIONS.forEach((theme) => {
        document.documentElement.classList.remove(`theme--${theme}`);
    });

    document.documentElement.classList.add(`theme--${themeName}`);
}

function onChangeThemeSelect() {
    setSavedTheme(themeSelect.value);
    setDocumentTheme(themeSelect.value);
}

function initializeFont() {
    let savedFont = getSavedFont();
    if (!savedFont) {
        setSavedFont(DEFAULT_FONT);
        savedFont = getSavedFont();
    }
    fontSelect.value = savedFont;
    setDocumentFont(savedFont);
}

function getSavedFont() {
    return localStorage.getItem('md_notes_font');
}

function setSavedFont(fontName) {
    localStorage.setItem('md_notes_font', fontName);
}

function setDocumentFont(fontName) {
    FONT_OPTIONS.forEach((font) => {
        document.documentElement.classList.remove(`font--${font}`);
    });

    document.documentElement.classList.add(`font--${fontName}`);
}

function onChangeFontSelect() {
    setSavedFont(fontSelect.value);
    setDocumentFont(fontSelect.value);
}

function initializeFontSize() {
  let savedFontSize = getSavedFontSize();
  if (!savedFontSize) {
    setSavedFontSize(DEFAULT_FONT_SIZE);
    savedFontSize = getSavedFontSize();
  }
  fontSizeInput.value = savedFontSize;
  setDocumentFontSize(savedFontSize);
}

function getSavedFontSize() {
  return localStorage.getItem('md_notes_font_size');
}

function setSavedFontSize(fontSize) {
  localStorage.setItem('md_notes_font_size', fontSize);
}

function setDocumentFontSize(fontSize) {
  document.documentElement.style.fontSize = `${fontSize}px`;
}

function onInputFontSize() {
  setSavedFontSize(fontSizeInput.value);
  setDocumentFontSize(fontSizeInput.value);
}

"#;

pub fn md_to_html(title: &str, body_content: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.tasklist = true;
    options.extension.table = true;
    options.extension.header_ids = Some("".to_string());
    options.render.r#unsafe = true;
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>{}</title>
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Libre+Baskerville:ital,wght@0,400;0,700;1,400&family=M+PLUS+1+Code:wght@100..700&family=Roboto+Mono:ital,wght@0,100..700;1,100..700&display=swap" rel="stylesheet">
        <style>{}</style>
    </head>
    <body>
        <main>
            <div class="toolbar">
                <select id="theme-select">
                    <option value="system">system</option>
                    <option value="light">light</option>
                    <option value="dark">dark</option>
                </select>
                <select id="font-select">
                    <option value="mono">mono</option>
                    <option value="sans">sans-serif</option>
                    <option value="serif">serif</option>
                </select>
                <input type="number" id="font-size-input" />
            </div>
            <div class='md'>
                {}
            </div>
        </main>
        <script>{}</script>
    </body>
</html>
"#,
        title,
        STYLES,
        comrak::markdown_to_html(&body_content, &options),
        SCRIPT
    )
}
