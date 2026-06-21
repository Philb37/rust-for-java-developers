package com.example.tickets.dto;

import java.util.Map;

import com.example.tickets.model.Priority;
import com.example.tickets.model.TicketStatus;

public record StatsResponse(
        Map<TicketStatus, Long> byStatus,
        Map<Priority, Long> byPriority
) {}
