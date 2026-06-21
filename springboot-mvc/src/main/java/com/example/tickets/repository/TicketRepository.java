package com.example.tickets.repository;

import java.util.List;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;
import org.springframework.data.jpa.repository.Query;

import com.example.tickets.model.Ticket;

public interface TicketRepository extends JpaRepository<Ticket, Integer>, JpaSpecificationExecutor<Ticket> {

    // Counts grouped by status AND priority in a single pass (Postgres GROUPING SETS),
    // instead of loading every row and grouping in memory.
    @Query(value = """
            SELECT status, priority, COUNT(*) AS count
            FROM tickets
            GROUP BY GROUPING SETS ((status), (priority))
            """, nativeQuery = true)
    List<StatCount> countByStatusAndPriority();
}
