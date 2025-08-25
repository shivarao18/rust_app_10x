use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct Ticket{
    pub id : u64,
    pub title : String,
}

#[derive(Deserialize)]
pub struct TicketforCreate{
    pub title : String,
}

#[derive(Clone)]
pub struct ModelController{
    ticket_store : Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController{
    pub fn new() -> Result<Self> {
        Ok(Self {
            ticket_store : Arc::new(Mutex::new(Vec::new())),
        })
    }
}

impl ModelController{
    pub fn create_ticket(&self, ticket_fc : TicketforCreate) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let id = store.len() as u64;
        let ticket = Ticket{
            id,
            title : ticket_fc.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets = store.iter().filter_map(|ticket| ticket.as_ref()).cloned().collect();

        Ok(tickets)
    }
    pub async fn delete_ticket(&self, id : u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound{id})
    }

}


;
