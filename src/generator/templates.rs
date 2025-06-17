#[derive(Debug)]
pub struct Templates {
    pub default: String,
}

impl Templates {
    pub fn new() -> Self {
        let default = String::from(
            r"export type Styles = {
{{#each class as |c| }}
  {{c}}: string;
{{/each}}
}

export type ClassNames = keyof Styles;

declare const styles: Styles;

export default styles;
",
        );

        Templates { default }
    }
}
