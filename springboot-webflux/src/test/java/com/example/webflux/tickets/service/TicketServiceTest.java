package com.example.webflux.tickets.service;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.ArgumentCaptor;
import static org.mockito.ArgumentMatchers.any;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;
import org.mockito.junit.jupiter.MockitoExtension;

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
                .thenReturn(Mono.just(ticket(1, TicketStatus.OPEN, Priority.HIGH)));

        TicketResponse response = service.create(
                new CreateTicketRequest("Printer on fire", Priority.HIGH, null, "Philippe")).block();

        ArgumentCaptor<Ticket> saved = ArgumentCaptor.forClass(Ticket.class);
        verify(repository).save(saved.capture());
        assertThat(saved.getValue().getTitle()).isEqualTo("Printer on fire");
        assertThat(saved.getValue().getPriority()).isEqualTo(Priority.HIGH);
        assertThat(saved.getValue().getAssignee()).contains("Philippe");
        // status is left null on purpose — TicketDefaultsCallback defaults it to OPEN at persistence time
        assertThat(saved.getValue().getStatus()).isNull();

        assertThat(response).isNotNull();
        assertThat(response.id()).isEqualTo(1);
        assertThat(response.status()).isEqualTo(TicketStatus.OPEN);
    }

    @Test
    void getByIdReturnsTicketWhenItExists() {
        when(repository.findById(1))
                .thenReturn(Mono.just(ticket(1, TicketStatus.OPEN, Priority.LOW)));

        TicketResponse response = service.getById(1).block();

        assertThat(response).isNotNull();
        assertThat(response.id()).isEqualTo(1);
        assertThat(response.title()).isEqualTo("Printer on fire");
    }

    @Test
    void getByIdThrowsWhenTicketDoesNotExist() {
        when(repository.findById(42)).thenReturn(Mono.empty());

        assertThatThrownBy(() -> service.getById(42).block())
                .isInstanceOf(TicketNotFoundException.class)
                .hasMessageContaining("42");
    }

    @Test
    void listMapsAllTicketsReturnedByTheRepository() {
        when(repository.findByFilters(TicketStatus.OPEN, Priority.HIGH))
                .thenReturn(Flux.just(
                        ticket(1, TicketStatus.OPEN, Priority.HIGH),
                        ticket(2, TicketStatus.OPEN, Priority.HIGH)));

        List<TicketResponse> responses = service.list(TicketStatus.OPEN, Priority.HIGH)
                .collectList().block();

        assertThat(responses).hasSize(2);
        assertThat(responses).extracting(TicketResponse::id).containsExactly(1, 2);
    }

    @Test
    void updateStatusSavesAndReturnsTheUpdatedTicket() {
        Ticket existing = ticket(1, TicketStatus.OPEN, Priority.MEDIUM);
        when(repository.findById(1)).thenReturn(Mono.just(existing));
        when(repository.save(existing)).thenReturn(Mono.just(existing));

        TicketResponse response = service.updateStatus(1, TicketStatus.RESOLVED).block();

        assertThat(existing.getStatus()).isEqualTo(TicketStatus.RESOLVED);
        assertThat(response).isNotNull();
        assertThat(response.status()).isEqualTo(TicketStatus.RESOLVED);
    }

    @Test
    void updateStatusThrowsWhenTicketDoesNotExist() {
        when(repository.findById(42)).thenReturn(Mono.empty());

        assertThatThrownBy(() -> service.updateStatus(42, TicketStatus.CLOSED).block())
                .isInstanceOf(TicketNotFoundException.class);
    }

    @Test
    void statsCountsTicketsByStatusAndPriority() {
        // Each GROUPING SETS row carries either a status or a priority — the other is null.
        when(repository.countByStatusAndPriority()).thenReturn(Flux.just(
                new StatCount("OPEN", null, 2L),
                new StatCount("CLOSED", null, 1L),
                new StatCount(null, "HIGH", 2L),
                new StatCount(null, "LOW", 1L)));

        StatsResponse stats = service.stats().block();

        assertThat(stats).isNotNull();
        assertThat(stats.byStatus()).isEqualTo(Map.of(
                TicketStatus.OPEN, 2L,
                TicketStatus.CLOSED, 1L));
        assertThat(stats.byPriority()).isEqualTo(Map.of(
                Priority.HIGH, 2L,
                Priority.LOW, 1L));
    }
}
