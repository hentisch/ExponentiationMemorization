pub trait Prioritizer{
    fn add_base(&self, base:i32, correct:bool);
    fn get_base(&self);
}