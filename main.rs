use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use html_editor::prelude::*;
use rphtml::config::{ParseOptions, RenderOptions};
use rphtml::parser::Doc;

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

fn main() -> Result<(), somehow::Error> {
    // fn main() -> Result<(), anyhow::Error> {
    // fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let index = reqwest::blocking::get("https://uupdump.net/")?.text()?;

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

    let render_html = doc.render(&RenderOptions {
        ..Default::default() // RenderOptions的参数定义可以在wiki中查看
    });
    let index = format!("{}", render_html);
    let index = html_editor::parse(&index)?;
    let selector = html_editor::Selector::from("table");
    let elements = index.query_all(&selector);
    let table = elements[0].html();

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
                            rows.push(Row {
                                build: build_text.to_string(),
                                arch: arch.to_string(),
                                date: date.to_string(),
                                id: id_text.to_string(),
                            })
                        }
                    }
                }
            }
        }
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&rows.clone())
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    if let Some(index) = selection {
        println!("{}", rows[index].id);
    }
    Ok(())
}
