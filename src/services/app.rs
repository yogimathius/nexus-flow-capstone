impl App {
    pub fn new(
        ai_client: AiClient,
        mcp_client: McpClient,
        config: Config,
        local_dream_interpreter: LocalDreamInterpreter,
    ) -> Self {
        let ai_client_with_mcp = ai_client.with_mcp_client(mcp_client.clone());

        App {
            ai_client: ai_client_with_mcp,
            mcp_client,
            config,
            local_dream_interpreter,
        }
    }
}
