use crate::domain::IdObject;

pub trait Entity {
    fn generate_id(&mut self) {
        let table_name = Self::get_table_name();
        self.get_mut_id().generate(table_name);
    }
    fn get_table_name() -> &'static str;
    fn get_id(&self) -> &IdObject;
    fn get_mut_id(&mut self) -> &mut IdObject;
}
