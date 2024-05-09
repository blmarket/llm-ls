# llm-ls - with daemonized backend

This is a drop-in replacement of
[llm-ls](https://github.com/huggingface/llm-ls)
with LLM daemon launching in a daemonized process.

## What is daemonized backend?

The LSP server spawns a LLM server in a forked process and let it run
indefinitely, eliminating the need for a user to launch another server such as
ollama etc.

Currently it requires llama.cpp to be installed on the system.

## Why daemonized backend?

You wanna run ollama (or other API endpoint) on your own? really?

## Configuration

Modify [server/model
location](https://github.com/blmarket/llm-ls/blob/main/crates/llama-daemon/src/daemon.rs#L30-L33)
in the source code, and build llm-ls in release mode.

Use following extensions to use compiled LSP server.

### llm-nvim

I'm using lazy.nvim, but it should also work with other plugin managers.

Note that current llm-daemon host a daemon with a port calculated from a hash,
which means you may need to set your own port number. (you can find port number
from /tmp/llm-{}.sock)

```lua
local M = {
  'huggingface/llm.nvim',
  opts = {
    backend = "ollama",
    url = "http://127.0.0.1:8080/completion",
    lsp = {
      bin_path = vim.fn.expand("$HOME/proj/llm-ls/target/release/llm-ls"),
      version = "0.5.2",
    },
    fim = {
      enabled = false,
    },
    context_window = 2048,
    enable_suggestions_on_startup = true,
  },
}

return M
```

### llm-vscode

```json
    "llm.lsp.binaryPath": "/home/blmarket/proj/llm-ls/target/release/llm-ls",
    "llm.backend": "ollama",
    "llm.url": "http://127.0.0.1:8080/completion",
    "llm.fillInTheMiddle.enabled": true,
    "llm.fillInTheMiddle.prefix": "<PRE> ",
    "llm.fillInTheMiddle.middle": " <MID>",
    "llm.fillInTheMiddle.suffix": " <SUF>",
```
