package com.example.tickets.service;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

import org.springframework.data.jpa.domain.Specification;
import org.springframework.stereotype.Service;

import com.example.tickets.dto.CreateTicketRequest;
import com.example.tickets.dto.StatsResponse;
import com.example.tickets.dto.TicketResponse;
import com.example.tickets.exception.TicketNotFoundException;
import com.example.tickets.model.Priority;
import com.example.tickets.model.Ticket;
import com.example.tickets.model.TicketStatus;
import com.example.tickets.repository.TicketRepository;
import com.example.tickets.repository.TicketSpecifications;

@Service
public class TicketService {

    private final TicketRepository repository;

    public TicketService(TicketRepository repository) {
        this.repository = repository;
    }

    public TicketResponse create(CreateTicketRequest request) {
        Ticket ticket = new Ticket();
        ticket.setTitle(request.title());
        ticket.setPriority(request.priority());
        ticket.setDescription(request.description());
        ticket.setAssignee(request.assignee());
        // status defaults to OPEN via @PrePersist

        Ticket saved = repository.save(ticket);
        return TicketResponse.from(saved);
    }

    public TicketResponse getById(Integer id) {
        return repository.findById(id)
                .map(TicketResponse::from)
                .orElseThrow(() -> new TicketNotFoundException(id));
        
    }

    public List<TicketResponse> list(TicketStatus status, Priority priority) {
        // Filtering happens in SQL, not in memory — Hibernate generates the WHERE clause
        Specification<Ticket> spec = TicketSpecifications.hasStatus(status)
                .and(TicketSpecifications.hasPriority(priority));

        return repository.findAll(spec).stream()
                .map(TicketResponse::from)
                .toList();
    }

    public TicketResponse updateStatus(Integer id, TicketStatus newStatus) {
        Ticket ticket = repository.findById(id)
                .orElseThrow(() -> new TicketNotFoundException(id));

        ticket.setStatus(newStatus);
        return TicketResponse.from(repository.save(ticket));
    }

    // Doing this in memory is bad practice, it should be done in SQL directly. 
    // The reason for it is to showcase how you could achieve something equivalent in rust 
    public StatsResponse stats() {
        List<Ticket> all = repository.findAll();

        Map<TicketStatus, Long> byStatus = all.stream()
                .collect(Collectors.groupingBy(
                        Ticket::getStatus,
                        Collectors.counting()
                ));

        Map<Priority, Long> byPriority = all.stream()
                .collect(Collectors.groupingBy(
                        Ticket::getPriority,
                        Collectors.counting()
                ));

        return new StatsResponse(byStatus, byPriority);
    }
}
