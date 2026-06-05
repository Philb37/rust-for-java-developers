

pub struct CreateTicketRequest {
    title: String,
    priority: Priority,
    description: Option<String>,
    assignee: Option<String>,
}