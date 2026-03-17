package com.example.tickets.dto;

import com.example.tickets.model.Priority;

// Java 21 record — immutable data carrier
// Rust parallel: struct CreateTicketRequest { title: String, priority: Priority, description: Option<String>, assignee: Option<String> }
public record CreateTicketRequest(
        String title,
        Priority priority,
        String description,  // null → Option::None
        String assignee      // null → Option::None
) {}
