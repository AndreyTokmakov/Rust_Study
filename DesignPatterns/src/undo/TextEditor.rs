
pub mod text_editor
{
    use std::rc::Rc;

    struct Document
    {
        content: String,
        version: u32,
    }

    struct Editor
    {
        current: Rc<Document>,
        history: Vec<Rc<Document>>,
    }

    impl Editor
    {
        fn new() -> Self {
            let doc: Rc<Document>
                = Rc::new(Document {
                content: String::new(),
                version: 1,
            });

            Editor {
                current: Rc::clone(&doc),
                history: vec![doc],
            }
        }

        fn edit(&mut self, newContent: String)
        {
            let newDoc: Rc<Document> = Rc::new(Document {
                content: newContent,
                version: self.current.version + 1,
            });

            self.history.push(Rc::clone(&newDoc));
            self.current = newDoc;
        }

        fn show_current(&self) {
            println!("Version {}: {}", self.current.version, self.current.content);
        }

        fn show_history(&self)
        {
            println!("History ({} versions):", self.history.len());
            for doc in &self.history {
                println!("  v{}: {}", doc.version, doc.content);
            }
        }
    }

    pub fn demo()
    {
        let mut editor: Editor = Editor::new();

        editor.edit("Hello".to_string());
        editor.edit("Hello, world!".to_string());
        editor.edit("Hello, Rust!".to_string());

        editor.show_current();
        editor.show_history();

        // Все версии хранятся в памяти без лишних копирований
        // Последняя версия имеет больше всего ссылок
        println!("Current ref count: {}", Rc::strong_count(&editor.current)); // 2 (current + history)
    }
}