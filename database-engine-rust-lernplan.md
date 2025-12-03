jjjj# Database Engine in Rust - 16 Stunden Lernplan

## Projekt Overview

**Ziel**: Implementierung einer funktionalen Mini-Database Engine in Rust  
**Zeitrahmen**: 16 Stunden (4 Sessions à 4 Stunden)  
**Schwerpunkt**: System Design, Architektur-Verständnis, Performance-optimierte Programmierung

---

## Phase 1: Storage Layer Foundation (Stunden 1-4)

### Aufgabe 1.1: Projekt Setup & B+ Tree Grundlagen (1h)

**Ziel**: Rust-Projekt initialisieren und B+ Tree Datenstruktur verstehen

1. Grundlegende B+ Tree Node-Strukturen definieren
2. Einfache Insert/Search Operationen implementieren

**Konkrete Schritte**:

```toml
# Cargo.toml Abhängigkeiten
[dependencies]
byteorder = "1.4"
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
```

**Hilfe-Prompt**:

```

### Aufgabe 1.2: B+ Tree Node Implementation (1.5h)

### Aufgabe 1.3: Page-based Storage System (1h)

**Ziel**: Disk-basiertes Storage mit fester Page-Größe

**Was zu tun ist**:

1. `Page` Struct mit 4KB Größe
2. `PageManager` für Allocation/Deallocation
3. Serialization/Deserialization von Nodes
4. Basic File I/O Operations

**Implementation Details**:

- Page Size: 4096 bytes
- Header: Page Type, Free Space, Key Count
- Payload: Keys + Values oder Child Pointers

**Hilfe-Prompt**:

```

Ich arbeite an dem Page Storage System (Phase 1.3). Problem bei [SERIALIZATION / FILE I/O / PAGE MANAGEMENT]:

Code:
[CODE]

Fehlerdetails: [ERROR MESSAGE]

Wie kann ich das lösen?

```

### Aufgabe 1.4: Integration & Basic Testing (0.5h)

**Ziel**: Storage Layer testen und debuggen

**Was zu tun ist**:

1. Unit Tests für Insert/Search
2. Integration Test mit File Persistence
3. Performance Test mit 1000+ Keys
4. Memory Usage Profiling

**Test Cases**:

- Sequential Inserts (1, 2, 3, ...)
- Random Inserts
- Duplicate Key Handling
- Tree Balance Verification

**Hilfe-Prompt**:

```

Meine Storage Layer Tests (Phase 1.4) schlagen fehl:

Test: [TEST NAME]
Error: [ERROR]
Code: [RELEVANT CODE]

Wie debugge ich das Problem und was läuft schief?

````

---

## Phase 2: SQL Parser (Stunden 5-8)

### Aufgabe 2.1: Tokenizer Implementation (1h)

**Ziel**: SQL-String in Token-Stream umwandeln

**Was zu tun ist**:

1. `Token` Enum für alle SQL-Elemente
2. `Lexer` Struct für Tokenization
3. Keywords erkennen (SELECT, FROM, WHERE, etc.)
4. Identifiers, Numbers, Strings parsen

**Unterstützte Tokens**:

```rust
enum Token {
    Keyword(String),      // SELECT, FROM, WHERE
    Identifier(String),   // column_name, table_name
    Number(i64),          // 123, -456
    String(String),       // 'hello'
    Operator(String),     // =, <, >, AND, OR
    Punctuation(char),    // (, ), ,, ;
}
````

**Hilfe-Prompt**:

```
Ich baue den SQL Tokenizer (Phase 2.1). Problem bei [KEYWORD RECOGNITION / STRING PARSING / NUMBER PARSING]:

Input SQL: [SQL STRING]
Current Code: [CODE]
Error: [ERROR DETAILS]

Wie kann ich das korrekt implementieren?
```

### Aufgabe 2.2: AST Definition (0.5h)

**Ziel**: Abstract Syntax Tree Strukturen für SQL

**Was zu tun ist**:

1. `Statement` Enum für verschiedene SQL-Typen
2. `SelectStatement` Struct mit Feldern
3. `Expression` für WHERE-Clauses
4. `Value` für Literals und Column References

**AST Struktur**:

```rust
enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    CreateTable(CreateTableStatement),
}

struct SelectStatement {
    columns: Vec<String>,
    table: String,
    where_clause: Option<Expression>,
}
```

**Hilfe-Prompt**:

```
Ich definiere die AST-Strukturen (Phase 2.2). Unsicher bei [AST DESIGN / EXPRESSION TYPES / STATEMENT STRUCTURE]:

Aktuelles Design: [CODE]

Ist das der richtige Ansatz für [SPEZIFISCHER ASPEKT]?
```

### Aufgabe 2.3: Recursive Descent Parser (2h)

**Ziel**: Token-Stream zu AST umwandeln

**Was zu tun ist**:

1. `Parser` Struct mit Token-Iterator
2. Parsing-Methoden für jedes Statement
3. Expression Parsing mit Operator Precedence
4. Error Handling und Recovery

**Parser Methods**:

- `parse_select()` → SelectStatement
- `parse_where()` → Option<Expression>
- `parse_expression()` → Expression
- `expect_token()` für Required Tokens

**Hilfe-Prompt**:

```
Mein Parser (Phase 2.3) hat Probleme bei [OPERATOR PRECEDENCE / ERROR HANDLING / SPECIFIC GRAMMAR RULE]:

SQL Input: [SQL]
Token Stream: [TOKENS]
Parser Code: [CODE]
Error: [ERROR]

Wie löse ich das Parsing-Problem?
```

### Aufgabe 2.4: Parser Integration Testing (0.5h)

**Ziel**: End-to-End SQL Parsing testen

**Test Cases**:

```sql
SELECT * FROM users;
SELECT name, email FROM users WHERE age > 25;
SELECT count(*) FROM products WHERE price < 100;
INSERT INTO users (name, age) VALUES ('Alice', 30);
CREATE TABLE products (id INT, name TEXT, price REAL);
```

**Hilfe-Prompt**:

```
Meine Parser-Tests (Phase 2.4) schlagen fehl:

SQL: [FAILING SQL]
Expected AST: [EXPECTED]
Actual Result: [ACTUAL/ERROR]

Wo liegt der Fehler in meinem Parser?
```

---

## Phase 3: Query Execution Engine (Stunden 9-12)

### Aufgabe 3.1: Table Schema & Record Management (1h)

**Ziel**: Table Definitions und Record Storage

**Was zu tun ist**:

1. `Schema` Struct für Column Definitions
2. `Record` für Row-Data mit typed Values
3. `Table` Manager für Schema + Storage Integration
4. Type System (INT, TEXT, REAL, BOOLEAN)

**Schema Definition**:

```rust
struct Schema {
    columns: Vec<Column>,
}

struct Column {
    name: String,
    data_type: DataType,
    nullable: bool,
}

enum DataType {
    Integer,
    Text,
    Real,
    Boolean,
}
```

**Hilfe-Prompt**:

```
Ich implementiere Table Schema Management (Phase 3.1). Problem bei [TYPE SYSTEM / RECORD SERIALIZATION / SCHEMA VALIDATION]:

Code: [CODE]
Error: [ERROR]

Wie soll ich [SPECIFIC ASPECT] richtig implementieren?
```

### Aufgabe 3.2: Query Execution Engine (1.5h)

**Ziel**: AST zu Execution Plan umwandeln

**Was zu tun ist**:

1. `Executor` Trait für verschiedene Operationen
2. `TableScanExecutor` für Full Table Scans
3. `FilterExecutor` für WHERE-Clauses
4. `ProjectionExecutor` für SELECT-Columns

**Executor Pattern**:

```rust
trait Executor {
    fn execute(&mut self) -> Result<Vec<Record>, Error>;
}

struct TableScanExecutor {
    table: Table,
}

struct FilterExecutor {
    input: Box<dyn Executor>,
    predicate: Expression,
}
```

**Hilfe-Prompt**:

```
Mein Query Executor (Phase 3.2) hat Probleme bei [TRAIT OBJECTS / EXECUTION PIPELINE / EXPRESSION EVALUATION]:

AST: [AST]
Executor Code: [CODE]
Issue: [PROBLEM DESCRIPTION]

Wie kann ich das lösen?
```

### Aufgabe 3.3: JOIN Implementation (1h)

**Ziel**: Nested Loop Join für Multi-Table Queries

**Was zu tun ist**:

1. `JoinExecutor` mit Left/Right Input Streams
2. Nested Loop Join Algorithm
3. JOIN Condition Evaluation
4. Result Record Construction

**Join Algorithm**:

```
for each record R in left_table:
    for each record S in right_table:
        if join_condition(R, S):
            yield combined_record(R, S)
```

**Hilfe-Prompt**:

```
Ich implementiere JOIN Operations (Phase 3.3). Problem bei [JOIN CONDITION / RECORD COMBINATION / PERFORMANCE]:

Tables: [TABLE SCHEMAS]
Join Code: [CODE]
Issue: [SPECIFIC PROBLEM]

Was mache ich falsch?
```

### Aufgabe 3.4: Index-based Query Optimization (0.5h)

**Ziel**: B+ Tree Indexes für WHERE-Clause Optimization

**Was zu tun ist**:

1. `IndexScanExecutor` für Key-based Lookups
2. Query Planner für Index Selection
3. Range Scans (>, <, BETWEEN)
4. Index vs Full Scan Cost Estimation

**Hilfe-Prompt**:

```
Die Index-Optimierung (Phase 3.4) funktioniert nicht richtig:

Query: [SQL QUERY]
Available Indexes: [INDEX INFO]
Current Plan: [EXECUTION PLAN]
Problem: [ISSUE]

Wie optimiere ich die Query-Ausführung?
```

---

## Phase 4: Transactions & Advanced Features (Stunden 13-16)

### Aufgabe 4.1: Write-Ahead Logging (WAL) (1.5h)

**Ziel**: Durability und Crash Recovery

**Was zu tun ist**:

1. `LogRecord` für verschiedene Operation Types
2. `WALManager` für Log-File Management
3. Before-Image Logging für Rollbacks
4. Checkpoint Mechanism

**Log Record Types**:

```rust
enum LogRecord {
    BeginTransaction(TransactionId),
    Insert { table: String, record: Record },
    Update { table: String, old: Record, new: Record },
    Delete { table: String, record: Record },
    Commit(TransactionId),
    Abort(TransactionId),
}
```

**Hilfe-Prompt**:

```
Ich implementiere WAL (Phase 4.1). Problem bei [LOG RECORD SERIALIZATION / CRASH RECOVERY / CHECKPOINT]:

Code: [CODE]
Error: [ERROR]
Scenario: [SPECIFIC SITUATION]

Wie implementiere ich [ASPECT] korrekt?
```

### Aufgabe 4.2: MVCC (Multi-Version Concurrency Control) (1.5h)

**Ziel**: Concurrency ohne Locking

**Was zu tun ist**:

1. Transaction IDs und Timestamps
2. Record Versioning (created_by, deleted_by)
3. Snapshot Isolation für Reads
4. Version Garbage Collection

**MVCC Record Structure**:

```rust
struct VersionedRecord {
    data: Record,
    created_by: TransactionId,
    deleted_by: Option<TransactionId>,
    timestamp: u64,
}
```

**Hilfe-Prompt**:

```
MVCC Implementation (Phase 4.2) bereitet mir Schwierigkeiten bei [SNAPSHOT ISOLATION / VERSION CLEANUP / TRANSACTION VISIBILITY]:

Transaction Scenario: [SCENARIO]
Code: [CODE]
Expected Behavior: [EXPECTED]
Actual Behavior: [ACTUAL]

Was läuft schief?
```

### Aufgabe 4.3: Query Optimization & Statistics (1h)

**Ziel**: Cost-based Query Planning

**Was zu tun ist**:

1. Table Statistics (Row Count, Distinct Values)
2. Histogram für Value Distribution
3. Cost Models für verschiedene Operationen
4. Query Plan Selection

**Cost Estimation**:

- Table Scan: O(n)
- Index Scan: O(log n + k)
- Nested Loop Join: O(n \* m)
- Hash Join: O(n + m)

**Hilfe-Prompt**:

```
Query Optimization (Phase 4.3) zeigt seltsame Ergebnisse:

Query: [SQL]
Available Plans: [PLANS]
Statistics: [STATS]
Chosen Plan: [SELECTED PLAN]
Issue: [PERFORMANCE PROBLEM]

Warum wählt der Optimizer den falschen Plan?
```

### Aufgabe 4.4: Integration Testing & Performance Benchmarks (1h)

**Ziel**: End-to-End Testing des kompletten Systems

**Test Scenarios**:

1. Concurrent Transactions (Read/Write Conflicts)
2. Crash Recovery (Kill Process during Transaction)
3. Performance Tests (1M Records, Complex Queries)
4. Memory Usage unter Load

**Benchmark Queries**:

```sql
-- Sequential Insert Performance
INSERT INTO bench_table (id, value) VALUES (1, 'test1');

-- Index vs Full Scan Comparison
SELECT * FROM large_table WHERE id = 12345;
SELECT * FROM large_table WHERE non_indexed_col = 'value';

-- Join Performance
SELECT * FROM table_a a JOIN table_b b ON a.id = b.foreign_id;

-- Transaction Throughput
BEGIN; UPDATE accounts SET balance = balance - 100 WHERE id = 1; COMMIT;
```

**Hilfe-Prompt**:

```
Integration Tests (Phase 4.4) zeigen Probleme:

Test Case: [TEST NAME]
Input Data: [DATA DESCRIPTION]
Expected: [EXPECTED RESULT]
Actual: [ACTUAL RESULT/ERROR]
Performance Metrics: [NUMBERS]

Wo liegt das Performance- oder Korrektheitsproblem?
```

---

## Erfolgs-Metriken

### Funktionale Anforderungen ✅

- [ ] CREATE TABLE, INSERT, SELECT Statements funktionieren
- [ ] WHERE-Clauses mit AND/OR/Vergleichsoperatoren
- [ ] JOIN zwischen zwei Tabellen
- [ ] B+ Tree Index-Scans
- [ ] ACID Transaktionen mit Rollback
- [ ] Crash Recovery über WAL

### Performance-Ziele 🚀

- [ ] 10,000 Sequential Inserts < 1 Sekunde
- [ ] Index Lookup < 1ms (für 100k Records)
- [ ] Join von 2 Tabellen (1k x 1k) < 100ms
- [ ] Transaction Throughput > 1000 TPS

### Code Quality 📊

- [ ] Modular Architecture (Storage/Parser/Executor getrennt)
- [ ] Comprehensive Error Handling
- [ ] Unit Tests für alle Major Components
- [ ] Memory Safety (keine unsafe{} Blocks nötig)
- [ ] Documentation für Public APIs

---

## Troubleshooting Guide

### Häufige Probleme & Lösungen

**Problem: Borrow Checker Fehler**

```
Lösung: Verwende Rc<RefCell<T>> für shared mutable state oder redesign ownership
Hilfe-Prompt: "Rust Borrow Checker Problem bei [SPECIFIC CODE], wie löse ich das?"
```

**Problem: Performance zu langsam**

```
Lösung: Profiling mit `cargo flamegraph`, dann Bottlenecks optimieren
Hilfe-Prompt: "Performance Problem: [BENCHMARK RESULTS], wo optimieren?"
```

**Problem: B+ Tree wird unbalanced**

```
Lösung: Check Split-Logik und Parent-Pointer Updates
Hilfe-Prompt: "B+ Tree Balance Problem: [TREE STATE], was ist falsch?"
```

**Problem: Parser erkennt SQL nicht**

```
Lösung: Debug Token-Stream, check Grammar Rules
Hilfe-Prompt: "Parser Problem mit SQL: [SQL], Token: [TOKENS], wo hängt es?"
```

**Problem: MVCC Deadlocks**

```
Lösung: Transaction Ordering, Timeout Mechanisms
Hilfe-Prompt: "MVCC Deadlock bei: [TRANSACTION SCENARIO], wie lösen?"
```

---

## Resources & Referenzen

### Bücher 📚

- "Database System Concepts" - Silberschatz, Galvin, Gagne
- "The Rust Programming Language" - Official Rust Book
- "Programming Rust" - Blandy, Orendorff, Tindall

### Online Resources 🌐

- [Rust Database Tutorial](https://github.com/erikgrinaker/toydb)
- [Build Your Own Database](https://build-your-own.org/database/)
- [CMU Database Course](https://15445.courses.cs.cmu.edu/)

### Tools 🛠️

- `cargo flamegraph` - Performance Profiling
- `valgrind` - Memory Leak Detection
- `rustfmt` - Code Formatting
- `clippy` - Linting und Best Practices

---

**Viel Erfolg bei deinem Database Engine Projekt! 🚀**

_Bei Problemen: Verwende die Hilfe-Prompts und beschreibe dein spezifisches Problem so detailliert wie möglich._
