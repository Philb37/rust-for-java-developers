package com.example.tickets.dto;

import com.example.tickets.model.Priority;
import com.example.tickets.model.TicketStatus;

import java.util.Map;

// Rust parallel: struct StatsResponse { by_status: HashMap<TicketStatus, u64>, by_priority: HashMap<Priority, u64> }
public record StatsResponse(
        Map<TicketStatus, Long> byStatus,
        Map<Priority, Long> byPriority
) {}
