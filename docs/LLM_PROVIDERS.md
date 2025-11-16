# LLM Providers Guide - mem0-rs

## Overview

mem0-rs supports multiple LLM providers for text generation and processing. Each provider has different capabilities, pricing, and performance characteristics.

## Supported Providers

### 1. Watsonx (IBM)

**Status**: ✅ Implemented

**Characteristics**:
- Enterprise-grade LLM service
- Multiple model options
- Streaming support
- Fine-tuning capabilities
- Strong privacy and security

**Setup**:
```bash
export WATSONX_API_KEY="your-api-key"
export WATSONX_PROJECT_ID="your-project-id"
```

**Usage**:
```rust
use mem0_rs::llm::watsonx::WatsonxLLM;

let llm = WatsonxLLM::with_defaults(
    api_key.to_string(),
    project_id.to_string(),
);

let response = llm.generate("What is AI?", None).await?;
```

**Best For**:
- Enterprise deployments
- Privacy-sensitive applications
- Custom model fine-tuning

### 2. OpenAI

**Status**: ✅ Implemented

**Characteristics**:
- State-of-the-art models (GPT-4, GPT-3.5)
- Excellent performance
- Wide adoption
- Competitive pricing
- Streaming support

**Setup**:
```bash
export OPENAI_API_KEY="your-api-key"
```

**Usage**:
```rust
use mem0_rs::llm::openai::OpenAILLM;

let llm = OpenAILLM::new("your-api-key".to_string());
let response = llm.generate("What is AI?", None).await?;
```

**With Custom Model**:
```rust
let llm = OpenAILLM::with_model(
    "your-api-key".to_string(),
    "gpt-3.5-turbo".to_string(),
);
```

**Available Models**:
- `gpt-4` - Most capable
- `gpt-4-turbo` - Faster GPT-4
- `gpt-3.5-turbo` - Fast and cost-effective
- `gpt-3.5-turbo-16k` - Extended context

**Best For**:
- Production applications
- High-quality outputs
- Wide model selection

**Pricing**: Pay-per-token based on model

### 3. Claude (Anthropic)

**Status**: ✅ Implemented

**Characteristics**:
- Constitutional AI approach
- Strong reasoning capabilities
- Long context window (200k tokens)
- Excellent safety features
- Competitive pricing

**Setup**:
```bash
export ANTHROPIC_API_KEY="your-api-key"
```

**Usage**:
```rust
use mem0_rs::llm::claude::ClaudeLLM;

let llm = ClaudeLLM::new("your-api-key".to_string());
let response = llm.generate("What is AI?", None).await?;
```

**With Custom Model**:
```rust
let llm = ClaudeLLM::with_model(
    "your-api-key".to_string(),
    "claude-3-sonnet-20240229".to_string(),
);
```

**Available Models**:
- `claude-3-opus-20240229` - Most capable
- `claude-3-sonnet-20240229` - Balanced
- `claude-3-haiku-20240307` - Fast and compact
- `claude-2.1` - Previous generation
- `claude-2` - Previous generation

**Best For**:
- Long-context applications
- Safety-critical tasks
- Reasoning-heavy workloads

**Pricing**: Pay-per-token based on model

### 4. Ollama

**Status**: 📋 Planned

**Characteristics**:
- Open-source LLM framework
- Run models locally
- No API costs
- Privacy-first
- Multiple model support

**Planned Usage**:
```rust
use mem0_rs::llm::ollama::OllamaLLM;

let llm = OllamaLLM::new("http://localhost:11434".to_string());
let response = llm.generate("What is AI?", None).await?;
```

### 5. Together AI

**Status**: 📋 Planned

**Characteristics**:
- Open-source model hosting
- Competitive pricing
- Multiple model options
- Fast inference
- Community-driven

**Planned Usage**:
```rust
use mem0_rs::llm::together::TogetherLLM;

let llm = TogetherLLM::new("your-api-key".to_string());
let response = llm.generate("What is AI?", None).await?;
```

### 6. Groq

**Status**: 📋 Planned

**Characteristics**:
- Ultra-fast inference
- Specialized hardware
- Competitive pricing
- Limited model selection
- Great for speed-critical apps

**Planned Usage**:
```rust
use mem0_rs::llm::groq::GroqLLM;

let llm = GroqLLM::new("your-api-key".to_string());
let response = llm.generate("What is AI?", None).await?;
```

## Provider Comparison

| Feature | Watsonx | OpenAI | Claude | Ollama | Together | Groq |
|---------|---------|--------|--------|--------|----------|------|
| Status | ✅ | ✅ | ✅ | 📋 | 📋 | 📋 |
| Cost | Paid | Paid | Paid | Free | Paid | Paid |
| Local | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |
| Streaming | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Context | 4k-20k | 4k-128k | 200k | Varies | Varies | 4k-32k |
| Speed | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Quality | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |

## Choosing a Provider

### Development
- **Recommended**: Ollama (local) or Claude (free tier)
- **Reason**: No costs, full control, good quality

### Small Production
- **Recommended**: Claude or OpenAI (gpt-3.5-turbo)
- **Reason**: Good balance of cost and quality

### Medium Production
- **Recommended**: OpenAI (gpt-4) or Claude (Sonnet)
- **Reason**: Excellent quality, reasonable cost

### Large Scale Production
- **Recommended**: Watsonx or OpenAI (gpt-4)
- **Reason**: Enterprise support, reliability, scalability

### Speed-Critical
- **Recommended**: Groq
- **Reason**: Ultra-fast inference

### Privacy-Critical
- **Recommended**: Ollama (local) or Watsonx
- **Reason**: Full data control

## Generation Parameters

### Basic Usage

```rust
use mem0_rs::llm::GenerationParams;

let params = GenerationParams {
    max_tokens: Some(500),
    temperature: Some(0.7),
    top_p: Some(0.9),
    top_k: None,
    stop_sequences: None,
};

let response = llm.generate("What is AI?", Some(params)).await?;
```

### Parameter Explanation

- **max_tokens**: Maximum length of generated text
- **temperature**: Randomness (0=deterministic, 1=random)
- **top_p**: Nucleus sampling threshold
- **top_k**: Top-k sampling
- **stop_sequences**: Sequences to stop generation

## Provider Configuration

### Using Environment Variables

```rust
let api_key = std::env::var("OPENAI_API_KEY")?;
let llm = OpenAILLM::new(api_key);
```

### Using Configuration Files

```rust
use mem0_rs::config::MemoryConfig;

let config = MemoryConfig::new(
    "http://localhost:6334".to_string(),
    std::env::var("WATSONX_API_KEY")?,
);
```

## Performance Characteristics

### Latency (ms)
- **Watsonx**: 500-2000ms
- **OpenAI**: 300-1500ms
- **Claude**: 400-1500ms
- **Ollama**: 100-500ms (local)
- **Together**: 200-1000ms
- **Groq**: 50-300ms

### Cost per 1k Tokens
- **Watsonx**: $0.01-0.10
- **OpenAI**: $0.0005-0.03
- **Claude**: $0.003-0.03
- **Ollama**: $0 (local)
- **Together**: $0.0005-0.01
- **Groq**: $0.0005-0.01

### Quality (1-5)
- **Watsonx**: 4.0
- **OpenAI**: 4.8
- **Claude**: 4.9
- **Ollama**: 3.0
- **Together**: 3.5
- **Groq**: 4.0

## Integration Example

### Multi-Provider Setup

```rust
use mem0_rs::llm::{LlmBase, OpenAILLM, ClaudeLLM};
use std::sync::Arc;

// Production
let openai = Arc::new(OpenAILLM::new(
    std::env::var("OPENAI_API_KEY")?
));

// Fallback
let claude = Arc::new(ClaudeLLM::new(
    std::env::var("ANTHROPIC_API_KEY")?
));

// Use with memory
let llm: Arc<dyn LlmBase> = openai;
let memory = Memory::new(config, vector_store, llm, embedder);
```

### Provider Selection

```rust
async fn select_provider(provider: &str) -> Result<Arc<dyn LlmBase>> {
    match provider {
        "openai" => Ok(Arc::new(OpenAILLM::new(
            std::env::var("OPENAI_API_KEY")?
        ))),
        "claude" => Ok(Arc::new(ClaudeLLM::new(
            std::env::var("ANTHROPIC_API_KEY")?
        ))),
        "watsonx" => Ok(Arc::new(WatsonxLLM::with_defaults(
            std::env::var("WATSONX_API_KEY")?,
            std::env::var("WATSONX_PROJECT_ID")?,
        ))),
        _ => Err("Unknown provider".into()),
    }
}
```

## Best Practices

1. **Use Environment Variables**
   - Store API keys securely
   - Never hardcode credentials
   - Use .env files for development

2. **Handle Errors Gracefully**
   - Implement retry logic
   - Have fallback providers
   - Log errors for debugging

3. **Monitor Usage**
   - Track token usage
   - Monitor costs
   - Set rate limits

4. **Optimize Prompts**
   - Keep prompts concise
   - Use system messages
   - Test different temperatures

5. **Cache Responses**
   - Cache common queries
   - Reduce API calls
   - Improve latency

## Troubleshooting

### Authentication Errors
- Verify API key is correct
- Check API key has required permissions
- Ensure API key is not expired

### Rate Limiting
- Implement exponential backoff
- Use request queuing
- Upgrade plan if needed

### Timeout Issues
- Increase timeout duration
- Check network connectivity
- Try different provider

### Quality Issues
- Adjust temperature parameter
- Refine prompt
- Try different model
- Use system message

## Future Providers

Planned providers for future releases:
- Ollama (Phase 9)
- Together AI (Phase 9)
- Groq (Phase 9)
- LiteLLM (Phase 10)
- Vertex AI (Phase 10)
- Azure OpenAI (Phase 10)

## See Also

- [README.md](README.md) - Main documentation
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Advanced features
- [STORAGE_BACKENDS.md](STORAGE_BACKENDS.md) - Storage backends
