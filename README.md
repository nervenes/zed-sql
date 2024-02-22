# zed-sql

Syntax highlighting for SQL in [Zed](https://github.com/zed-industries/zed).

### Test locally

- Clone this repo: `git clone https://github.com/evrsen/zed-sql sql`
- Clone the [tree-sitter-sql](https://github.com/evrsen/tree-sitter-sql) repo: `https://github.com/evrsen/tree-sitter-sql`
- CD into the repo: `cd tree-sitter-sql`
- Build the WASM: `tree-sitter build-wasm` (might require docker-engine running)
- Rename the WASM file to `sql.wasm`
- Move the WASM file into `sql/grammars` (this repository)
- Move the `sql`repository to `~/Library/Application Support/Zed/extensions/installed`
