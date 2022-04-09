use html_editor::prelude::*;
use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::Doc;
use rphtml::types::HResult;

// fn main() -> Result<(), anyhow::Error> {
fn main() -> HResult {
    let index = reqwest::blocking::get("https://uupdump.net/")?.text()?;
    // println!("{}", resp);
    //fn main() -> Result<(), String> {
    // let index = include_str!("index.html");
    // let index = prettyish_html::prettify(index);
    // let index = html_editor::parse(&index)?;
    // let index = html_parser::Dom::parse(index)?;

    let doc = Doc::parse(
        &index,
        ParseOptions {
            case_sensitive_tagname: false, // 解析时标签区分大小写，`<div>` 和 `<DIV>` 将被视作不同标签，不建议开启
            allow_self_closing: true,      // 允许非替换元素使用自闭合的写法，如 `<div class='' />`
            auto_fix_unclosed_tag: true,   // 自动修复没有结束的标签，注意这里只是简单的将标签闭合
            auto_fix_unexpected_endtag: true, // 自动修复不正确的结束标签，如 "<div></p></div>" 会被修复为 "<div></div>"
            auto_fix_unescaped_lt: true, // 自动修复没有实体转译的左尖括号 '<', 比如`<div>a<b</div>`会被修复为`<div>a&lt;b</div>`
        },
    )?;
    /*
    let index = scraper::Html::parse_document(index);
    let selector = scraper::Selector::parse("html body div.pusher div.ui.container table.ui.striped.table").unwrap();
    let table = index.select(&selector).next().unwrap();
    let text = table.text().collect::<Vec<_>>();
    println!("{:?}", table);
    */
    // println!("{:?}", doc);
    // let root = doc.get_root_node();
    let render_html = doc.render(&RenderOptions {
        ..Default::default() // RenderOptions的参数定义可以在wiki中查看
    });
    let index = format!("{}", render_html);
    let index = html_editor::parse(&index)?;
    let selector = html_editor::Selector::from("table");
    let elements = index.query_all(&selector);
    let table = elements[0].html();

    // println!("{}", table);
    let table = table_extract::Table::find_by_headers(&table, &["Build"]).unwrap();
    let mut rows = Vec::<Row>::new();
    for row in &table {
        if let [build, arch, date, id] = row.iter().collect::<Vec<_>>()[..] {
            let builds = html_editor::parse(&build)?;
            if let [_, html_editor::Node::Element {
                name: _,
                attrs: _,
                children: nodes,
            }] = &builds[..]
            {
                if let html_editor::Node::Text(ref build_text) = nodes[0] {
                    let id = html_editor::parse(&id)?;
                    if let html_editor::Node::Element {
                        name: _,
                        attrs: _,
                        children: nodes,
                    } = &id[0]
                    {
                        if let html_editor::Node::Text(ref id_text) = nodes[0] {
                            // println!("{build_text: <80} {id_text: <44} {date}");
                            // println!("{build_text: <80} {date}");
                            rows.push(Row {
                                build: build_text.to_string(),
                                arch: arch.to_string(),
                                date: date.to_string(),
                                id: id_text.to_string(),
                            })
                        }
                    }
                }
                // println!("{arch} {date} {id}")
                // println!("{:?}", &builds[1]);
                // println!("{build} {arch} {date} {id}")
            }
        }

        /*
        for cell in row {
            println!("Table cell: {}", cell);
        }
        */
    }
    // let R: Vec<Row> = rows.clone();
    /*
        if let Some(row) = select_one(rows.into_iter()) {
            println!("{}", row.text());
        }
    */
    {
        use dialoguer::console::Term;
        use dialoguer::{theme::ColorfulTheme, Select};
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&rows.clone())
            .default(0)
            .interact_on_opt(&Term::stderr())?;
        if let Some(index) = selection {
            println!("{}", rows[index].id);
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Row {
    pub build: String,
    pub arch: String,
    pub date: String,
    pub id: String,
}

impl ToString for Row {
    fn to_string(&self) -> String {
        format!(
            "{: <8} {: <80} {: <44} {}",
            self.arch, self.build, self.id, self.date
        )
        .clone()
    }
}

impl std::convert::AsRef<str> for &Row {
    fn as_ref(&self) -> &str {
        let a = &self.clone().build;
        &a
    }
}

impl skim::SkimItem for Row {
    fn text(&self) -> skim::prelude::Cow<str> {
        skim::prelude::Cow::from(self.id.clone())
    }
    fn display(&self, _: skim::DisplayContext) -> skim::AnsiString {
        format!(
            "{: <80} {: <8} {} {}",
            self.build, self.arch, self.date, self.id
        )
        .into()
    }
}

/// select one from an iterator of SkimItem
///
/// <https://github.com/DanSnow/github-release-download/blob/99d876ca3acc869d13e3dda0c0558cef6ad4acc5/src/bin/grd.rs>
use std::sync::Arc;
pub fn select_one<T: skim::SkimItem, I: IntoIterator<Item = T>>(
    items: I,
) -> Option<Arc<dyn skim::SkimItem>> {
    let items = items
        .into_iter()
        .map(|x| Arc::new(x) as Arc<dyn skim::SkimItem>)
        .collect::<Vec<Arc<dyn skim::SkimItem>>>();
    let (tx, rx) = skim::prelude::bounded(32);
    std::thread::spawn(move || {
        for item in items {
            if tx.send(item).is_err() {
                break;
            }
        }
    });
    let items = skim::Skim::run_with(&skim::SkimOptions::default(), Some(rx));
    if let Some(ref inner) = items {
        if inner.is_abort {
            return None;
        }
    }
    let items = items
        .map(|output| output.selected_items)
        .unwrap_or_else(Vec::new);
    items.into_iter().next()
}
