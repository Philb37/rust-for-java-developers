package com.example.webflux.tickets.dto;

import java.util.Map;

import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.TicketStatus;

public record StatsResponse(
        Map<TicketStatus, Long> byStatus,
        Map<Priority, Long> byPriority
) {}
