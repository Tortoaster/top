use crate::component::Component;
use crate::editor::Editor;

#[derive(Debug)]
pub struct SequentialEditor<E> {
    editor: Option<E>,
    buttons: Vec<String>,
}

impl<E> SequentialEditor<E> {
    pub fn with_editor(mut self, editor: E) -> Self {
        self.editor = Some(editor);
        self
    }

    pub fn with_button(mut self, button: String) -> Self {
        self.buttons.push(button);
        self
    }
}

impl<E> Editor for SequentialEditor<E>
where
    E: Editor,
{
    type Read = E::Read;
    type Write = E::Write;
    type Error = E::Error;

    fn new() -> Self {
        SequentialEditor {
            editor: None,
            buttons: Vec::new(),
        }
    }

    fn ui(&self) -> Component {
        Component::Column(vec![
            self.editor.as_ref().unwrap().ui(),
            Component::Row(
                self.buttons
                    .iter()
                    .cloned()
                    .map(|text| Component::Button {
                        text,
                        disabled: false,
                    })
                    .collect(),
            ),
        ])
    }

    fn read_value(&self) -> Self::Read {
        self.editor.as_ref().unwrap().read_value()
    }

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error> {
        self.editor.as_mut().unwrap().write_value(value)
    }
}
