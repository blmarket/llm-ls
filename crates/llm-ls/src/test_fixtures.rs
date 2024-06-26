use custom_types::llm_ls::{Backend, FimParams, GetCompletionsParams, Ide};
use serde_json::Map;
use tower_lsp::lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams};

pub fn test_get_params(backend: Backend) -> GetCompletionsParams {
    GetCompletionsParams {
        api_token: None,
        context_window: 100,
        fim: FimParams {
            enabled: false,
            prefix: "".into(),
            middle: "".into(),
            suffix: "".into(),
        },
        ide: Ide::Neovim,
        model: "model".to_string(),
        backend,
        text_document_position: TextDocumentPositionParams {
            position: Position {
                line: 0,
                character: 0,
            },
            text_document: TextDocumentIdentifier {
                uri: reqwest::Url::parse("file:///").unwrap(),
            },
        },
        tls_skip_verify_insecure: false,
        tokens_to_clear: vec![],
        tokenizer_config: None,
        request_body: Map::new(),
    }
}
