package com.example.tickets.repository;

// Interface projection for the GROUPING SETS aggregate query. Spring Data maps each
// selected column to the matching getter by name (status -> getStatus, etc.).
// Each row carries EITHER a status or a priority — the other one is null.
public interface StatCount {
    String getStatus();

    String getPriority();

    long getCount();
}
