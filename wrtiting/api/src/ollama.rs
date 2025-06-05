pub async fn send_prompt_to_ollama(prompt: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "llama3",
            "prompt": prompt,
            "stream": false,
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}