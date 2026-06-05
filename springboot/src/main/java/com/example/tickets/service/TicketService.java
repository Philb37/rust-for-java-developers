package com.example.tickets.service;

import com.example.tickets.dto.CreateTicketRequest;
import com.example.tickets.dto.StatsResponse;
import com.example.tickets.dto.TicketResponse;
import com.example.tickets.exception.TicketNotFoundException;
import com.example.tickets.model.Priority;
import com.example.tickets.model.Ticket;
import com.example.tickets.model.TicketStatus;
import com.example.tickets.repository.TicketRepository;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

// Rust parallel: impl TicketService — a struct with methods
@Service
public class TicketService {

    private final TicketRepository repository;

    public TicketService(TicketRepository repository) {
        this.repository = repository;
    }

    // Rust parallel: fn create(req: CreateTicketRequest) -> Result<TicketResponse, AppError>
    public TicketResponse create(CreateTicketRequest request) {
        Ticket ticket = new Ticket();
        ticket.setTitle(request.title());
        ticket.setPriority(request.priority());
        ticket.setDescription(request.description()); // null → Option::None
        ticket.setAssignee(request.assignee());        // null → Option::None
        // status defaults to OPEN via @PrePersist

        Ticket saved = repository.save(ticket);
        return TicketResponse.from(saved);
    }

    // Rust parallel: fn get_by_id(id: u64) -> Result<TicketResponse, TicketNotFoundError>
    public TicketResponse getById(Integer id) {
        return repository.findById(id)          // Optional<Ticket>  ←→  Option<Ticket>
                .map(TicketResponse::from)      // Optional<TicketResponse>
                .orElseThrow(() -> new TicketNotFoundException(id));
        // Rust: .ok_or(TicketNotFoundError(id))?
    }

    // Rust parallel: fn list(status: Option<TicketStatus>, priority: Option<Priority>) -> Vec<TicketResponse>
    public List<TicketResponse> list(TicketStatus status, Priority priority) {
        List<Ticket> all = repository.findAll();

        return all.stream()
                // Rust: .filter(|t| status.map_or(true, |s| t.status == s))
                .filter(t -> status == null || t.getStatus() == status)
                // Rust: .filter(|t| priority.map_or(true, |p| t.priority == p))
                .filter(t -> priority == null || t.getPriority() == priority)
                // Rust: .map(TicketResponse::from)
                .map(TicketResponse::from)
                // Rust: .collect::<Vec<_>>()
                .toList();
    }

    // Rust parallel: fn update_status(id: u64, status: TicketStatus) -> Result<TicketResponse, TicketNotFoundError>
    public TicketResponse updateStatus(Integer id, TicketStatus newStatus) {
        Ticket ticket = repository.findById(id)
                .orElseThrow(() -> new TicketNotFoundException(id));

        ticket.setStatus(newStatus);
        return TicketResponse.from(repository.save(ticket));
    }

    // Rust parallel: fn stats() -> StatsResponse
    public StatsResponse stats() {
        List<Ticket> all = repository.findAll();

        // Rust: tickets.iter().counts_by(|t| t.status)
        Map<TicketStatus, Long> byStatus = all.stream()
                .collect(Collectors.groupingBy(
                        Ticket::getStatus,     // key extractor  ←→  closure |t| t.status
                        Collectors.counting()  // downstream     ←→  count per group
                ));

        // Rust: tickets.iter().counts_by(|t| t.priority)
        Map<Priority, Long> byPriority = all.stream()
                .collect(Collectors.groupingBy(
                        Ticket::getPriority,
                        Collectors.counting()
                ));

        return new StatsResponse(byStatus, byPriority);
    }
}
