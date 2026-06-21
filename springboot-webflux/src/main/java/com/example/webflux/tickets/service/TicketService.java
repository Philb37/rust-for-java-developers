package com.example.webflux.tickets.service;

import java.util.EnumMap;
import java.util.Map;

import org.springframework.stereotype.Service;

import com.example.webflux.tickets.dto.CreateTicketRequest;
import com.example.webflux.tickets.dto.StatsResponse;
import com.example.webflux.tickets.dto.TicketResponse;
import com.example.webflux.tickets.exception.TicketNotFoundException;
import com.example.webflux.tickets.model.Priority;
import com.example.webflux.tickets.model.Ticket;
import com.example.webflux.tickets.model.TicketStatus;
import com.example.webflux.tickets.repository.StatCount;
import com.example.webflux.tickets.repository.TicketRepository;

import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@Service
public class TicketService {

    private final TicketRepository repository;

    public TicketService(TicketRepository repository) {
        this.repository = repository;
    }

    public Mono<TicketResponse> create(CreateTicketRequest request) {
        Ticket ticket = new Ticket();
        ticket.setTitle(request.title());
        ticket.setPriority(request.priority());
        ticket.setDescription(request.description());
        ticket.setAssignee(request.assignee());
        // status / createdAt default in TicketDefaultsCallback before the row is written

        return repository.save(ticket).map(TicketResponse::from);
    }

    public Mono<TicketResponse> getById(Integer id) {
        return repository.findById(id)
                .map(TicketResponse::from)
                .switchIfEmpty(Mono.error(() -> new TicketNotFoundException(id)));
    }

    public Flux<TicketResponse> list(TicketStatus status, Priority priority) {
        return repository.findByFilters(status, priority)
                .map(TicketResponse::from);
    }

    public Mono<TicketResponse> updateStatus(Integer id, TicketStatus newStatus) {
        return repository.findById(id)
                .switchIfEmpty(Mono.error(() -> new TicketNotFoundException(id)))
                .flatMap(ticket -> {
                    ticket.setStatus(newStatus);
                    return repository.save(ticket);
                })
                .map(TicketResponse::from);
    }

    // The GROUPING SETS query lives in the repository; the service just folds its rows.
    public Mono<StatsResponse> stats() {
        return repository.countByStatusAndPriority()
                .collectList()
                .map(rows -> {
                    Map<TicketStatus, Long> byStatus = new EnumMap<>(TicketStatus.class);
                    Map<Priority, Long> byPriority = new EnumMap<>(Priority.class);

                    for (StatCount row : rows) {
                        if (row.status() != null) {
                            byStatus.put(TicketStatus.valueOf(row.status()), row.count());
                        } else if (row.priority() != null) {
                            byPriority.put(Priority.valueOf(row.priority()), row.count());
                        }
                    }
                    return new StatsResponse(byStatus, byPriority);
                });
    }
}
