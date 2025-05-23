# Solana Analytics System Implementation Plan

## Current Version
- Version: 0.1.0
- Status: Core library and tests build and pass; examples pending update
- Last Updated: 2024-04-XX

## Recent Progress (2024-04-XX)
- Main library and all tests now build and pass.
- `HealthStatus` refactored to an enum (`Healthy`, `Unhealthy`).
- Trait implementations updated for new API.
- Legacy Solana SDK error usage removed from tests.
- Integration and unit tests migrated to best-practice structure.
- Only the example code remains to be updated for the new API.

## Key Features
1. Progressive Complexity
   - Start with basic infrastructure
   - Build up to advanced analytics
   - Layer features incrementally
   - Maintain backward compatibility

2. Dependency Management
   - Precise Rust crate specifications
   - Version pinning for stability
   - Feature flag management
   - Dependency conflict resolution

3. Error Handling
   - Comprehensive error types
   - Contextual error information
   - Graceful degradation
   - Error recovery strategies

4. Testing Integration
   - Unit tests for all components
   - Integration tests for workflows
   - Performance benchmarks
   - Property-based testing

5. Performance Focus
   - Real-time processing optimization
   - Resource usage monitoring
   - Caching strategies
   - Load testing requirements

## Implementation Rules
1. Research-first approach for each component
2. Document findings before implementation
3. Follow Rust best practices and idioms
4. Maintain high test coverage
5. Document all public APIs
6. Handle errors gracefully
7. Optimize for performance
8. Keep code modular and maintainable
9. Follow TDD best practices

## Progress
- [x] Core traits and interfaces defined
- [x] Basic RPC client structure implemented
- [x] Configuration system implemented
- [x] Health monitoring system implemented
- [x] Rate limiting system implemented
- [x] Developer QA instructions for test coverage and API documentation added
- [x] Review similar open-source projects
- [x] Compile reference documentation
- [x] Document specific requirements for each phase
- [x] Create technical specifications
- [x] Set up documentation structure
- [x] Architecture diagrams created and added to documentation
- [x] Core data model skeletons implemented
- [x] Core indexer skeleton implemented
- [ ] Retry mechanism implemented
- [ ] Connection pooling implemented
- [ ] Error handling improvements
- [ ] Comprehensive testing
- [ ] Documentation (final review)

### Phase 0: Research and Documentation (REQUIRED BEFORE ANY IMPLEMENTATION)
#### Project Analysis
- [x] Document QA workflow and developer quality assurance instructions
- [x] Review current codebase structure
  - [x] Document existing architecture
  - [x] Map component dependencies
  - [x] Identify technical debt
  - [x] List current limitations
- [x] Document existing patterns and conventions
  - [x] Code organization patterns
  - [x] Error handling approaches
  - [x] Testing methodologies
  - [x] Documentation standards
- [x] Identify current dependencies and their purposes
  - [x] List all external crates
  - [x] Document version requirements
  - [x] Note feature flags in use
  - [x] Map dependency relationships
- [x] Map out current system interactions
  - [x] Document data flows
  - [x] Identify integration points
  - [x] Note performance bottlenecks
  - [x] List security considerations

#### Technology Research
- [x] Research best practices for each component
  - RPC client patterns
  - Database access patterns
  - Error handling strategies
  - Testing methodologies
- [x] Document relevant Rust patterns and idioms
  - [x] Async/await patterns
  - [x] Error handling idioms
  - [x] Testing best practices
  - [x] Performance optimization techniques
- [x] Review similar open-source projects
  - Analyze architecture decisions
  - Document successful patterns
  - Note failure points
  - List lessons learned
- [x] Compile reference documentation
  - Create technical specifications
  - Document design decisions
  - List implementation guidelines
  - Note security considerations

#### Requirements Documentation
- [x] Document specific requirements for each phase
  - Functional requirements
  - Non-functional requirements
  - Performance requirements
  - Security requirements
- [x] Create technical specifications
  - API specifications
  - Data models
  - Error handling
  - Testing requirements
- [x] Define success criteria
  - Performance metrics
  - Test coverage goals
  - Documentation standards
  - Security requirements
- [x] Set up documentation structure
  - API documentation
  - Architecture documentation
  - Development guides
  - Deployment guides

### Phase 1: Core Infrastructure Foundation
#### RPC Client Infrastructure
- [ ] Build `SolanaRpcClient` with multiple endpoint support
- [ ] Implement connection pooling with reqwest
- [ ] Add retry logic with exponential backoff
- [ ] Implement rate limiting with governor
- [ ] Add health monitoring and failover
- [ ] Create configuration system

#### Database Layer Setup
- [ ] Set up PostgreSQL with sqlx
- [ ] Create database connection pool manager
- [ ] Design and implement schema migrations
- [ ] Add database health checks
- [ ] Create indexing strategy
- [ ] Implement connection retry logic

#### Core Data Models
- [ ] Define core structs with serde
- [ ] Create custom error types
- [ ] Build configuration structs
- [ ] Add validation traits
- [ ] Implement Display and Debug traits

### Phase 2: On-Chain Data Extraction
#### Transaction Fetching Engine
- [ ] Create `TransactionFetcher` struct
- [ ] Implement pagination handling
- [ ] Add parallel processing
- [ ] Add progress tracking
- [ ] Implement checkpointing
- [ ] Add error handling

#### Program-Specific Parsers
- [ ] Build trait-based parsing system
- [ ] Implement Token Program parser
- [ ] Add DeFi protocol parsers
- [ ] Create instruction categorization
- [ ] Add Metaplex Token Metadata parsing
- [ ] Implement comprehensive testing

#### Balance Tracking System
- [ ] Create `BalanceTracker`
- [ ] Implement balance snapshots
- [ ] Add historical balance reconstruction
- [ ] Handle edge cases
- [ ] Implement efficient storage
- [ ] Add balance validation

### Phase 3: External Data Integration
#### Price Feed System
- [ ] Create `PriceProvider` trait
- [ ] Implement multiple price sources
- [ ] Build `PriceAggregator`
- [ ] Add caching layer
- [ ] Implement rate limiting
- [ ] Add fallback mechanisms

#### Token Metadata Resolution
- [ ] Build `TokenRegistry`
- [ ] Implement metadata fetching
- [ ] Add caching strategy
- [ ] Implement validation
- [ ] Handle different sources
- [ ] Add security checks

### Phase 4: Analytics Engine
#### Transaction Metrics Calculator
- [ ] Create `TransactionAnalyzer`
- [ ] Implement frequency analysis
- [ ] Add value distribution analysis
- [ ] Create categorization system
- [ ] Add cost analysis
- [ ] Implement advanced analytics

#### Portfolio Analytics Engine
- [ ] Build `PortfolioAnalyzer`
- [ ] Implement value calculations
- [ ] Add diversity metrics
- [ ] Create holding period analysis
- [ ] Implement PnL calculations
- [ ] Add advanced portfolio features

#### DeFi Protocol Analysis
- [ ] Create `DeFiAnalyzer`
- [ ] Implement protocol-specific analyzers
- [ ] Add key metrics calculation
- [ ] Create advanced DeFi analysis
- [ ] Implement risk calculations
- [ ] Handle complex scenarios

### Phase 5: Advanced Analytics
#### Risk Assessment Engine
- [ ] Create `RiskAssessment` system
- [ ] Implement leverage analysis
- [ ] Add risk metrics
- [ ] Create insurance integration
- [ ] Add real-time monitoring
- [ ] Implement risk scoring

#### Governance Participation Tracker
- [ ] Build `GovernanceTracker`
- [ ] Add supported governance systems
- [ ] Implement key metrics
- [ ] Create advanced analysis
- [ ] Add DAO participation scoring
- [ ] Implement cross-DAO analysis

#### Behavioral Analysis System
- [ ] Create `BehaviorAnalyzer`
- [ ] Implement risk tolerance inference
- [ ] Add sophistication scoring
- [ ] Create learning progression tracking
- [ ] Implement behavioral clustering
- [ ] Add machine learning integration

### Phase 6: Real-Time Processing
#### Event-Driven Architecture
- [ ] Create `EventProcessor`
- [ ] Implement WebSocket connections
- [ ] Add event queue processing
- [ ] Create real-time analytics
- [ ] Implement webhook system
- [ ] Add monitoring

#### Background Processing System
- [ ] Build `JobProcessor`
- [ ] Implement task queue management
- [ ] Add scheduling system
- [ ] Create worker management
- [ ] Implement job persistence
- [ ] Add monitoring

### Phase 7: API & Service Layer
#### REST API Development
- [ ] Create API with Axum
- [ ] Implement core endpoints
- [ ] Add authentication system
- [ ] Optimize responses
- [ ] Create API documentation
- [ ] Add monitoring

#### GraphQL Layer (Optional)
- [ ] Create GraphQL schema
- [ ] Implement resolvers
- [ ] Add subscription support
- [ ] Optimize performance
- [ ] Implement security
- [ ] Add documentation

### Phase 8: Monitoring & Operations
#### Observability System
- [ ] Implement structured logging
- [ ] Add metrics collection
- [ ] Create health monitoring
- [ ] Implement alerting system
- [ ] Create dashboards
- [ ] Add error tracking

#### Configuration & Deployment
- [ ] Create configuration management
- [ ] Implement Docker containerization
- [ ] Add Kubernetes deployment
- [ ] Create database management
- [ ] Implement CI/CD pipeline
- [ ] Add production considerations

## Critical Path for MVP
1. Core Infrastructure (Phase 1)
   - RPC Client Infrastructure (1.1)
   - Database Layer Setup (1.2)
   - Core Data Models (1.3)

2. Data Extraction (Phase 2)
   - Transaction Fetching Engine (2.1)
   - Program-Specific Parsers (2.2)
   - Balance Tracking System (2.3)

3. Basic Analytics (Phase 4)
   - Transaction Metrics Calculator (4.1)
   - Portfolio Analytics Engine (4.2)

4. Price Integration (Phase 3)
   - Price Feed System (3.1)
   - Token Metadata Resolution (3.2)

5. Real-Time Processing (Phase 6)
   - Event-Driven Architecture (6.1)
   - Background Processing System (6.2)

6. API Access (Phase 7)
   - REST API Development (7.1)

## Research Findings
*This section will be updated as research progresses, documenting:*
- Technical decisions and rationale
- Alternative approaches considered
- Performance implications
- Security considerations
- Integration challenges
- Testing strategies
- Documentation requirements

## Implementation Decisions
*This section will be updated as decisions are made, including:*
- Architecture choices
- Technology selections
- Design patterns
- Testing approaches
- Documentation standards
- Performance optimizations
- Security measures

## Next Steps
1. Complete Phase 0 research
   - Finish project analysis
   - Complete technology research
   - Document all requirements
   - Set up documentation structure
2. Review and validate findings
   - Verify technical decisions
   - Validate performance assumptions
   - Check security implications
   - Confirm testing strategies
3. Document implementation plan
   - Create detailed specifications
   - Define success criteria
   - Set up monitoring
   - Plan testing approach
4. Begin Phase 1 implementation
   - Start with core infrastructure
   - Follow research findings
   - Implement incrementally
   - Document progress

**Note:**
- Developer QA instructions for test coverage and API documentation are now available in this document. All contributors should follow these as part of the development workflow.

## Development Strategy
1. Follow the sequence
   - Each phase builds on previous ones
   - Validate dependencies before proceeding
   - Maintain clear upgrade paths
   - Document integration points

2. Customize as needed
   - Add specific business requirements
   - Adapt to performance needs
   - Scale based on usage patterns
   - Optimize for target use cases

3. Test incrementally
   - Validate each component
   - Test integration points
   - Verify performance metrics
   - Document test coverage

4. Deploy progressively
   - Start with core functionality
   - Add features incrementally
   - Monitor system stability
   - Gather user feedback

## Notes
- Complete research phase before implementation
- Document all findings and decisions
- Implement one component at a time
- Write tests before implementation
- Include comprehensive testing
- Add robust error handling
- Consider performance implications
- Add proper documentation
- Keep track of progress
- Update document regularly
- Monitor resource usage
- Track technical debt
- Regular security audits
- Performance benchmarking
- Code quality checks
- Documentation updates
- Validate assumptions
- Document research sources
- Review findings regularly
- Update based on new information

# Codebase Review Summary (Phase 0)

## 1. Code Organization & Modularity
- **src/rpc/**: Solana RPC client, configuration, error handling, health monitoring, rate limiting, and tests.
- **src/core/**: Foundational traits, error handling, configuration, health, logging, utilities, and test helpers.
- **src/models/**: Data models for transactions, tokens, protocols, price history, and governance.
- **src/db/**: Database models, logic, and migrations.
- **Top-level**: Project manifest, lockfile, README, implementation plan, Docker, and config files.

## 2. Error Handling Consistency
- Error handling is robust and modularized.
- Uses `thiserror` for custom error enums in both core and RPC modules.
- Errors are contextual, support conversion from external errors, and are separated by concern.
- Patterns are consistent with Rust best practices.

## 3. Testing Coverage & Quality
- Dedicated test files and test utilities are present.
- Integration test setup for the database is provided.
- The implementation plan mandates comprehensive unit and integration tests.
- README includes clear testing instructions.
- **Recommendation:** Run a coverage tool (e.g., `cargo tarpaulin`) to quantify and improve test coverage.

## 4. Dependency Usage & Best Practices
- Dependencies are well-organized in `Cargo.toml` with clear feature flags.
- Uses modern, well-maintained crates for async, error handling, validation, and database access.
- Dev-dependencies are separated for testing and benchmarking.
- Optional dependencies and features are used to keep the build lean.

## 5. Documentation & Comments
- Module-level and inline documentation is present in several files.
- The `README.md` is detailed and up to date.
- The `IMPLEMENTATION_PLAN.md` is comprehensive, emphasizing research and documentation before implementation.
- **Recommendation:** Ensure all public APIs are documented and generate API docs with `cargo doc`.

## 6. Technical Debt
- No explicit TODO, FIXME, or deprecated markers found in the codebase.
- No commented-out code detected in the search.
- **Recommendation:** Continue to monitor for technical debt and document any future issues.

## 7. Security & Performance Considerations
- Security and performance are addressed in the implementation plan.
- Rate limiting, connection pooling, and async patterns are in use.
- Logging and metrics are integrated for observability.

## 8. Current Limitations
- Some advanced features (real-time, analytics, API) are not yet implemented.
- Test coverage should be measured and improved if needed.
- API documentation should be kept up to date as the codebase evolves.

# Developer Quality Assurance Instructions

## Test Coverage Analysis

To quantify and improve test coverage, use [cargo-tarpaulin](https://github.com/xd009642/tarpaulin):

1. Install tarpaulin (if not already installed):
   ```sh
   cargo install cargo-tarpaulin
   ```
2. Run the coverage analysis:
   ```sh
   cargo tarpaulin --all-features --workspace --out Html
   ```
   This will generate a coverage report and an HTML file you can open in your browser.

## API Documentation Generation

To ensure all public APIs are documented and generate browsable docs:

1. Generate the documentation:
   ```sh
   cargo doc --all-features --workspace --no-deps --open
   ```
   This will build the docs and open them in your browser.

**Recommendation:**
- Review the coverage and documentation reports regularly.
- Address any uncovered code or undocumented public APIs as part of the development workflow.

---

**Reviewed on:** 2025-05-23  
**Reviewer:** funt3ars

## Technology Research: Best Practices for Each Component

### 1. RPC Client Patterns
- Connection Management: Use connection pooling (e.g., reqwest), support multiple endpoints with failover and load balancing.
- Retry Logic: Implement exponential backoff (e.g., tokio-retry), make parameters configurable.
- Rate Limiting: Use token bucket algorithm (governor crate), allow per-endpoint and global limits.
- Error Handling: Use rich error types (thiserror, anyhow), propagate context, distinguish retryable/fatal errors.
- Async/Await: Use async/await for all network ops, leverage tokio runtime.
- Observability: Integrate tracing, expose health endpoints and metrics.

### 2. Database Access Patterns
- Connection Pooling: Use async pools (sqlx, deadpool-postgres), configure for production.
- Migrations: Use migration tools (sqlx-cli), version control schema.
- Error Handling: Map DB errors to custom types, handle connection loss/retry.
- Async Operations: Use async queries/transactions.
- Testing: Use test DBs and transactional rollbacks for integration tests.
- Indexing & Performance: Design indexes for time-based/high-frequency queries, monitor and optimize.

### 3. Error Handling Strategies
- Error Types: Use thiserror for custom enums, anyhow for aggregation.
- Error Context: Always add context to errors.
- Propagation: Use Result<T, E> everywhere, avoid panics in libraries.
- Separation: Separate recoverable from fatal errors.
- Logging: Log errors with context.
- Testing: Test error paths and edge cases.

### 4. Testing Methodologies
- Unit Testing: Test logic-heavy functions/modules in isolation.
- Integration Testing: Test end-to-end flows (DB, RPC).
- Mocking: Use mockall for trait-based mocking.
- Property-Based Testing: Use proptest for critical logic.
- Coverage: Use cargo tarpaulin to measure/improve coverage.
- CI Integration: Run tests/coverage in CI.
- Performance Testing: Use criterion for benchmarking.

**References:**
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [tokio.rs](https://tokio.rs/)
- [sqlx Documentation](https://docs.rs/sqlx/)
- [Governor Crate](https://docs.rs/governor/)
- [thiserror](https://docs.rs/thiserror/)
- [mockall](https://docs.rs/mockall/)
- [cargo-tarpaulin](https://docs.rs/cargo-tarpaulin/)
- [criterion.rs](https://docs.rs/criterion/)

## Technology Research: Rust Patterns and Idioms

### Async/Await Patterns
- Use `async fn` and `.await` for all I/O-bound and concurrent operations.
- Prefer `tokio` as the async runtime for ecosystem compatibility.
- Use `tokio::spawn` for lightweight task concurrency.
- Use `futures::stream` for batching and streaming data.
- Avoid blocking calls in async contexts; use `tokio::task::spawn_blocking` for CPU-bound work.

### Error Handling Idioms
- Use `Result<T, E>` for all fallible operations.
- Use `thiserror` for custom error enums and `anyhow` for error aggregation at the application boundary.
- Add context to errors using `.context()` (from `anyhow`) or custom error variants.
- Avoid panics in library code; prefer graceful error propagation.
- Use `?` operator for concise error propagation.

### Testing Best Practices
- Use `#[cfg(test)]` and `mod tests` for unit tests within modules.
- Use integration tests in the `tests/` directory for end-to-end scenarios.
- Use `mockall` for trait-based mocking in unit tests.
- Use `proptest` for property-based testing of critical logic.
- Use `test-log` or `tracing` for capturing logs during tests.

### Performance Optimization Techniques
- Use `Arc` and `RwLock` for shared, concurrent state.
- Minimize locking granularity to reduce contention.
- Use `tokio::sync` primitives for async synchronization.
- Profile with `cargo-flamegraph` and benchmark with `criterion`.
- Use zero-copy deserialization (e.g., `serde` with `borrow` attributes) for large data.

### Idiomatic Rust Code
- Prefer iterators and combinators over manual loops.
- Use pattern matching for control flow and error handling.
- Leverage enums for state machines and protocol handling.
- Use `Option` and `Result` types for explicit null/error handling.
- Document all public APIs with `///` doc comments.

**References:**
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

## Review of Similar Open-Source Projects (2025-05)

### 1. helius-labs/photon (Rust)
- High-performance Solana indexer with snapshot support, Postgres/SQLite, and modular CLI configuration.
- **Key patterns:**
  - Modular, production-grade Rust codebase
  - Connection pooling, automated migrations, snapshotting
  - Clear separation between core indexer and program-specific logic
  - CLI-driven configuration
- **Lessons:**
  - Snapshotting and migration automation are critical for reliability and fast recovery
  - Extensibility for custom program analytics is important
  - Documentation and CLI usability are strong points

### 2. Tee-py/solana-txn-parser (TypeScript)
- Modular transaction parser for DeFi protocols (PumpFun, Raydium, Jupiter).
- **Key patterns:**
  - Extensible base parser for protocol-specific logic
  - Action/event modeling for analytics
  - Community contribution guidelines and unit tests
- **Lessons:**
  - Protocol-specific parsing should be modular and easily extendable
  - Action/event modeling is key for analytics
  - Clear contribution and testing guidelines foster community involvement

### 3. stella3d/solana-data (Rust)
- CLI tool for scraping, chunking, and analyzing Solana data (file-based, plans for DB backend).
- **Key patterns:**
  - Task-based CLI for modular workflows
  - Simple, extensible design
- **Lessons:**
  - File-based storage is simple but not scalable; DB integration is essential for production
  - Modular CLI tasks are effective for data workflows

### 4. sanoagent/Unified_Solana_Analysis_and_Portfolio_Evaluation_Pipeline (Jupyter)
- Portfolio evaluation pipeline; not production-grade but may inspire analytics features.

---

### How These Findings Influence Our Implementation
- **Rust-first, modular architecture**: We will maintain a clear separation between core indexing, protocol-specific parsing, and analytics modules.
- **Connection pooling, migrations, and snapshotting**: Adopt best practices for DB reliability and fast recovery.
- **Extensible parser framework**: Design protocol-specific parsers as modular, pluggable components.
- **Action/event modeling**: Use event-driven data models for analytics, inspired by solana-txn-parser.
- **CLI-driven configuration and documentation**: Prioritize usability and maintainability.
- **Testing and automation**: Ensure robust test coverage and automated migrations.

---

### Next Steps
- Incorporate these patterns and lessons into technical specifications and requirements documentation.

## Expanded Technical Specifications and Implementation Guidelines

### 1. System Architecture (Detailed)
- **Core Indexer**: Responsible for ingesting blocks and transactions from Solana RPC, dispatching to protocol-specific parsers.
- **Parser Modules**: Each protocol (e.g., SPL Token, DeFi, Governance) has its own parser implementing a common trait/interface.
- **Analytics Engine**: Consumes parsed events/actions, computes metrics, aggregates, and stores analytics results.
- **Database Layer**: Handles all persistent storage, migrations, and connection pooling.
- **Health & Monitoring**: Exposes health endpoints, tracks RPC/DB status, and logs metrics.
- **Configuration Loader**: Loads and validates config from file, env, and CLI.

### 2. Data Models (Detailed)
- **SolanaTransaction**: Represents a full transaction, including signatures, instructions, logs, and metadata.
- **TokenAccount**: Models SPL token accounts, balances, and ownership.
- **PriceData**: Stores price feed data with source, timestamp, and confidence.
- **ProtocolInteraction**: Captures DeFi/governance protocol events (e.g., swaps, votes).
- **GovernanceVote**: Models on-chain governance voting events.
- **Traits**: All models derive Serialize, Deserialize, Validate, Debug, Display, and have UUIDs and timestamps.

### 3. Error Handling (Detailed)
- **Error Types**: RpcError, DatabaseError, PriceProviderError, AnalyticsError, ConfigError, ValidationError.
- **Error Propagation**: Use `thiserror` and `anyhow` for rich error context.
- **Retry Logic**: Implement exponential backoff for transient errors (RPC, DB, price feeds).
- **Graceful Degradation**: System continues operating in degraded mode if some sources fail.

### 4. Configuration (Detailed)
- **Config Structs**: Separate structs for each subsystem (RPC, DB, API, analytics, health).
- **Validation**: Use `validator` crate for all config fields.
- **Overrides**: Support for environment variables and CLI flags.
- **Hot Reload (Future)**: Plan for dynamic config reloads without restart.

### 5. Validation (Detailed)
- **Model Validation**: All models implement a `validate()` method.
- **Config Validation**: Performed at startup; system aborts on invalid config.
- **Data Integrity**: Enforce unique constraints and referential integrity in DB.

### 6. Database Integration (Detailed)
- **PostgreSQL via sqlx**: Async pool, migrations, health checks.
- **Schema**: Normalized tables for transactions, accounts, prices, protocol events, governance votes.
- **Indexes**: On frequently queried fields (slot, signature, account, protocol, timestamp).
- **Batch Operations**: Use transactions for bulk inserts/updates.
- **Migration Strategy**: All schema changes via versioned migration files.

### 7. Testing (Detailed)
- **Unit Tests**: For all models, parsers, error types, and config.
- **Integration Tests**: End-to-end tests for RPC ingestion, parsing, DB storage, and analytics.
- **Property-Based Tests**: For protocol parsers and analytics logic.
- **Performance Benchmarks**: For ingestion, parsing, and DB operations.
- **Test Automation**: CI runs all tests and migrations on each commit.

### 8. Documentation (Detailed)
- **Inline Rustdoc**: All public structs, traits, and functions documented.
- **README**: Usage, configuration, architecture, and contribution guidelines.
- **Architecture Docs**: Diagrams and flowcharts for data flows and module interactions.
- **Developer Guides**: How to add new protocols, migrations, or analytics.

### 9. Non-Functional Requirements (Detailed)
- **Performance**: Ingest and process at least 1000 transactions/sec on commodity hardware.
- **Reliability**: 99.9% uptime, automated failover, and recovery.
- **Security**: Principle of least privilege for DB and RPC, input validation, and secure config.
- **Extensibility**: New protocols and analytics can be added with minimal changes to core.
- **Observability**: Metrics, logging, and alerting for all critical paths.

---

## API Endpoints, Data Flows, and Success Criteria

### API Endpoints (Planned)
- **/health**: Returns system health, DB/RPC status, and metrics.
- **/transactions**: Query transactions by slot, signature, account, or time range.
- **/accounts**: Query token accounts and balances.
- **/prices**: Query historical and real-time price data.
- **/protocol-events**: Query protocol-specific events (swaps, votes, etc.).
- **/analytics**: Query computed analytics (e.g., volume, user stats, governance outcomes).

### Data Flows
1. **Ingestion**: Core indexer fetches blocks/transactions from Solana RPC.
2. **Parsing**: Transactions dispatched to protocol-specific parsers.
3. **Event Modeling**: Parsers emit events/actions, which are validated and stored.
4. **Analytics**: Analytics engine computes metrics from stored events and transactions.
5. **API**: Data is served via REST/gRPC endpoints for external consumers.
6. **Monitoring**: Health and metrics endpoints provide observability.

### Success Criteria
- **Functional**:
  - All endpoints return correct, validated data.
  - System ingests and parses Solana mainnet data in real time.
  - Protocol-specific analytics are accurate and up-to-date.
- **Non-Functional**:
  - >99.9% uptime, automated recovery from failures.
  - Ingestion and query latency < 500ms for 95% of requests.
  - Test coverage >90% for all critical modules.
  - Documentation is complete and up-to-date.

---

### Next Steps
- Begin implementation of detailed technical specifications, starting with core indexer and data models.
- Draft OpenAPI/Protobuf specs for API endpoints.
- Create architecture diagrams for documentation.

### Next Step
- Implement and test model validation logic, or expand the indexer to wire up a basic protocol parser.

## Next Steps
- Update all example code in `examples/` to match the current API (field names, types, methods).
- Continue with planned feature development and documentation review.