package com.example.tickets.service;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;
import java.util.Optional;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.ArgumentCaptor;
import org.mockito.ArgumentMatchers;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;
import org.springframework.data.jpa.domain.Specification;

import com.example.tickets.dto.CreateTicketRequest;
import com.example.tickets.dto.StatsResponse;
import com.example.tickets.dto.TicketResponse;
import com.example.tickets.exception.TicketNotFoundException;
import com.example.tickets.model.Priority;
import com.example.tickets.model.Ticket;
import com.example.tickets.model.TicketStatus;
import com.example.tickets.repository.TicketRepository;

// Plain Mockito unit test — no Spring context, no database
@ExtendWith(MockitoExtension.class)
class TicketServiceTest {

    @Mock
    private TicketRepository repository;

    @InjectMocks
    private TicketService service;

    private static Ticket ticket(Integer id, TicketStatus status, Priority priority) {
        Ticket ticket = new Ticket();
        ticket.setId(id);
        ticket.setTitle("Printer on fire");
        ticket.setStatus(status);
        ticket.setPriority(priority);
        ticket.setCreatedAt(OffsetDateTime.now());
        return ticket;
    }

    @Test
    void createMapsRequestAndReturnsSavedTicket() {
        when(repository.save(any(Ticket.class)))
                .thenReturn(ticket(1, TicketStatus.OPEN, Priority.HIGH));

        TicketResponse response = service.create(
                new CreateTicketRequest("Printer on fire", Priority.HIGH, null, "Philippe"));

        ArgumentCaptor<Ticket> saved = ArgumentCaptor.forClass(Ticket.class);
        verify(repository).save(saved.capture());
        assertThat(saved.getValue().getTitle()).isEqualTo("Printer on fire");
        assertThat(saved.getValue().getPriority()).isEqualTo(Priority.HIGH);
        assertThat(saved.getValue().getAssignee()).contains("Philippe");
        // status is left null on purpose — @PrePersist defaults it to OPEN
        assertThat(saved.getValue().getStatus()).isNull();

        assertThat(response.id()).isEqualTo(1);
        assertThat(response.status()).isEqualTo(TicketStatus.OPEN);
    }

    @Test
    void getByIdReturnsTicketWhenItExists() {
        when(repository.findById(1))
                .thenReturn(Optional.of(ticket(1, TicketStatus.OPEN, Priority.LOW)));

        TicketResponse response = service.getById(1);

        assertThat(response.id()).isEqualTo(1);
        assertThat(response.title()).isEqualTo("Printer on fire");
    }

    @Test
    void getByIdThrowsWhenTicketDoesNotExist() {
        when(repository.findById(42)).thenReturn(Optional.empty());

        assertThatThrownBy(() -> service.getById(42))
                .isInstanceOf(TicketNotFoundException.class)
                .hasMessageContaining("42");
    }

    @Test
    void listMapsAllTicketsReturnedByTheSpecification() {
        when(repository.findAll(ArgumentMatchers.<Specification<Ticket>>any()))
                .thenReturn(List.of(
                        ticket(1, TicketStatus.OPEN, Priority.HIGH),
                        ticket(2, TicketStatus.OPEN, Priority.HIGH)));

        List<TicketResponse> responses = service.list(TicketStatus.OPEN, Priority.HIGH);

        assertThat(responses).hasSize(2);
        assertThat(responses).extracting(TicketResponse::id).containsExactly(1, 2);
    }

    @Test
    void updateStatusSavesAndReturnsTheUpdatedTicket() {
        Ticket existing = ticket(1, TicketStatus.OPEN, Priority.MEDIUM);
        when(repository.findById(1)).thenReturn(Optional.of(existing));
        when(repository.save(existing)).thenReturn(existing);

        TicketResponse response = service.updateStatus(1, TicketStatus.RESOLVED);

        assertThat(existing.getStatus()).isEqualTo(TicketStatus.RESOLVED);
        assertThat(response.status()).isEqualTo(TicketStatus.RESOLVED);
    }

    @Test
    void updateStatusThrowsWhenTicketDoesNotExist() {
        when(repository.findById(42)).thenReturn(Optional.empty());

        assertThatThrownBy(() -> service.updateStatus(42, TicketStatus.CLOSED))
                .isInstanceOf(TicketNotFoundException.class);
    }

    @Test
    void statsCountsTicketsByStatusAndPriority() {
        when(repository.findAll()).thenReturn(List.of(
                ticket(1, TicketStatus.OPEN, Priority.HIGH),
                ticket(2, TicketStatus.OPEN, Priority.LOW),
                ticket(3, TicketStatus.CLOSED, Priority.HIGH)));

        StatsResponse stats = service.stats();

        assertThat(stats.byStatus()).isEqualTo(Map.of(
                TicketStatus.OPEN, 2L,
                TicketStatus.CLOSED, 1L));
        assertThat(stats.byPriority()).isEqualTo(Map.of(
                Priority.HIGH, 2L,
                Priority.LOW, 1L));
    }
}
