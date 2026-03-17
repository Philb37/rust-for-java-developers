package com.example.tickets.repository;

import com.example.tickets.model.Priority;
import com.example.tickets.model.Ticket;
import com.example.tickets.model.TicketStatus;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

// Rust parallel: a trait that defines data access behavior
// JpaRepository gives us findById() which returns Optional<Ticket> — the clearest Option<T> parallel
public interface TicketRepository extends JpaRepository<Ticket, Long> {

    // Rust parallel: fn find_by_status(status: TicketStatus) -> Vec<Ticket>
    List<Ticket> findByStatus(TicketStatus status);

    List<Ticket> findByPriority(Priority priority);

    List<Ticket> findByStatusAndPriority(TicketStatus status, Priority priority);
}
