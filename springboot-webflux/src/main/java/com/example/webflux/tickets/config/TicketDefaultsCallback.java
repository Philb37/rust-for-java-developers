package com.example.webflux.tickets.config;

import java.time.OffsetDateTime;

import org.reactivestreams.Publisher;
import org.springframework.data.r2dbc.mapping.event.BeforeConvertCallback;
import org.springframework.data.relational.core.sql.SqlIdentifier;
import org.springframework.stereotype.Component;

import com.example.webflux.tickets.model.Ticket;
import com.example.webflux.tickets.model.TicketStatus;

import reactor.core.publisher.Mono;

/**
 * Applies persistence-time defaults to a {@link Ticket} before it is converted to a row.
 *
 * <p>This is the Spring Data R2DBC equivalent of JPA's {@code @PrePersist}: it runs on the
 * persistence boundary rather than in the service. Both defaults are guarded by null checks,
 * so the callback is idempotent and leaves already-persisted tickets untouched on update.
 */
@Component
public class TicketDefaultsCallback implements BeforeConvertCallback<Ticket> {

    @Override
    public Publisher<Ticket> onBeforeConvert(Ticket ticket, SqlIdentifier table) {
        if (ticket.getCreatedAt() == null) {
            ticket.setCreatedAt(OffsetDateTime.now());
        }
        if (ticket.getStatus() == null) {
            ticket.setStatus(TicketStatus.OPEN);
        }
        return Mono.just(ticket);
    }
}
