# Multi-tenant SaaS Backend

A robust, scalable multi-tenant SaaS application backend built with **Rust**, featuring high-performance networking, reliable data isolation, and a modern developer experience.


## 🏗 Why and Where Each Concept Is Used

| Concept | Location | Purpose |
| :--- | :--- | :--- |
| **RLS (Row-Level Security)** | All tables | **Tenant Isolation**: Ensures Company A can never access Company B's data at the database level. |
| **Optimistic Locking** | `PUT /tasks` | **Concurrency Protection**: Detects and prevents data loss if two users edit the same task simultaneously. |
| **Full-text Search** | `GET /tasks?search=` | **Efficient Discovery**: Allows users to search task titles and descriptions with advanced relevance. |
| **GIN Index** | `tasks.search_vector` | **Search Optimization**: Makes complex full-text search queries extremely fast. |
| **JSONB + GIN** | `tasks.metadata` | **Dynamic Schemas**: Supports flexible custom fields per-tenant without modifying the global schema. |
| **Recursive CTE** | `GET /tasks/:id/subtasks` | **Hierarchical Data**: Fetches an entire tree of nested subtasks in a single efficient query. |
| **SKIP LOCKED** | Job Worker | **Parallel Execution**: Allows multiple workers to safely process jobs concurrently without double-claiming. |
| **Redis Cache-Aside** | `GET /projects` | **Performance**: Reduces database load for frequently accessed, rarely changing data. |
| **Redis Pub/Sub** | WebSocket Updates | **Real-time Sync**: Broadcasts task updates and alerts to all connected users instantly. |
| **SQLx Migrations** | Schema Management | **Consistency**: Ensures version-controlled, reproducible schema updates across all environments. |
