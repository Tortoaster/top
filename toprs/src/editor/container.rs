use crate::component::Component;
use crate::editor::Editor;

#[derive(Debug)]
pub struct SequentialEditor<E> {
    editor: E,
    button: String,
}

impl<E> SequentialEditor<E> {
    pub fn new(editor: E, button: String) -> Self {
        SequentialEditor { editor, button }
    }
}

impl<E> Editor for SequentialEditor<E>
where
    E: Editor,
{
    type Read = E::Read;
    type Write = E::Write;
    type Error = E::Error;

    fn ui(&self) -> Component {
        Component::Column(vec![
            self.editor.ui(),
            Component::Button {
                text: self.button.clone(),
                disabled: false,
            },
        ])
    }

    fn read_value(&self) -> Self::Read {
        self.editor.read_value()
    }

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error> {
        self.editor.write_value(value)
    }
}
