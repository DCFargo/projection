#[derive(Savefile)]
pub(crate) enum KanbanStatus {
    Todo,
    Progress,
    Done,
}

#[derive(Savefile)]
pub(crate) struct KanbanObject {
    pub ref_id: String,
    pub display_name: String,
    pub status: KanbanStatus,
    pub scheduled: bool,
    pub year: u32,
    pub month: u8,
    pub day: u32,
    pub hour: u8,
}

#[derive(Savefile)]
pub(crate) struct GoalObject {
    pub ref_id: String,
    pub display_name: String,
    pub complete: bool,
}
